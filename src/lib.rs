mod common;
mod mc_utils;
mod proxy;

use icrate::Foundation::{NSHomeDirectory, NSString, ns_string};
use objc2::rc::{autoreleasepool, Id};
use once_cell::sync::Lazy;
use proxy::LSBundleProxy;
use simplelog::{Config, WriteLogger, LevelFilter};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Mutex;
use std::{fs, thread};
use ctor::ctor;


static SHADER_PATHS: Lazy<Mutex<HashMap<OsString, PathBuf>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn get_path() -> PathBuf {
    unsafe {
    let bundle_proxy =
        LSBundleProxy::bundleProxyForIdentifier(ns_string!("com.mojang.minecraftpreview"));
    
        let data_path = bundle_proxy.dataContainerUrl();
        let path_str = data_path.path().unwrap();
        let rspath_buf = path_str.to_string();
        let mut base_path = PathBuf::from(rspath_buf);
    base_path.push("Documents/games/com.mojang/minecraftpe");
    base_path
    }
}
// TODO: Figure out if unwinding is the devil in this scenario
// unwinding beyond startup might be ub so idk 
#[ctor]
fn startup() {
    let path = get_path();
    let Ok(logfile) = fs::File::create(path.join("mbsr_log.txt")) else {
        return;
    };
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        logfile);

    log::info!("Rust thread starting up!");
    log::info!("Hi if you see this everything is fine");
    log::info!("It wont work because im reimplementing stuff");
    let _handler = thread::spawn(|| {
        common::setup_json_watcher(path)
    }).join();
    
    //Somewhat insane but ill take it
    if let Ok(watch_err) = _handler {
        if let Err(e) = watch_err {
        log::error!("Watcher setup failed: {e}");
        }
    }
}

// This can be done safely thanks to objc2 which tries very hard to make 
// types zero sized and also make them compile as lowly as possible
fn mbsr_get_rep_path(
    file_name: &NSString,
    ext_name: &NSString
) -> Id<NSString> {
    // TODO: figure out how to return null to avoid retaining
    // empty strings, which is VERY bad to do when this is used to
    // get path to assets which might be run many times
    let result = autoreleasepool(|pool| {
        let rs_filename = file_name.as_str(pool);
        let extension = ext_name.as_str(pool);
        log::info!(target: "MBHook", "Operating on path: {rs_filename:#?}");
        if !rs_filename.ends_with(".material.bin") 
        && extension != "material.bin" {
            return log_dismiss("Path is not for a material bin {rs_filename:#?}");
        }
        let path = Path::new(rs_filename);
        let Some(filename) = path.file_name() else {
            return log_dismiss("Cant parse filename from path {rs_filename:#?}");
        };
        let Ok(shader_paths) = SHADER_PATHS.lock() else {
            return log_dismiss("Preventing crash: {e:#?}");
        };
        if let Some(rep_path) = shader_paths.get(filename)  {
            let Some(utf8_path) = rep_path.to_str() else {
                return log_dismiss("Custom path is not utf8: {rep_path:#?}");
            };
            return NSString::from_str(utf8_path);
        };
        log_dismiss("Dismissing path: {rs_filename:#?}")
    });
    result
}

#[inline(always)]
fn log_dismiss(reason: &str) -> Id<NSString>{
    log::warn!("{reason}");
    //Really wanting to get rid of this, objc calls are expensive
    NSString::from_str("")
}
