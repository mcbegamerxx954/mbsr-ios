mod common;
mod mc_utils;

use icrate::Foundation::{NSHomeDirectory, NSString};
use objc2::rc::{autoreleasepool, Id};
use once_cell::sync::Lazy;
use oslog::OsLogger;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread;

static SHADER_PATHS: Lazy<Mutex<HashMap<OsString, PathBuf>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn get_path() -> PathBuf {
        let mut pathbuf = std::env::home_dir().unwrap();
        pathbuf.extend(["Documents", "games", "com.mojang", "minecraftpe"]);
        pathbuf
}
#[no_mangle]
unsafe extern "C" fn mbsr_ruststartup() {
    OsLogger::new("com.minecraft")
        .level_filter(log::LevelFilter::Debug)
        .init()
        .unwrap();

    log::info!(target: "ModInit", "Rust thread starting up!");
    let _handler = thread::spawn(|| {
        common::setup_json_watcher(get_path());
    }).join();
    
    match _handler {
        Ok(_) => (),
        Err(e) => match e.downcast_ref::<&'static str>() {
            Some(e_str) => log::error!(target:"ModInit", "JsonWatch thread Error: {e_str}"),
            None => log::error!(target: "ModInit", "Unknown error in thread.."),
        },
    }
}

// This can be done safely thanks to objc2 which tries very hard to make 
// types zero sized and also make them compile as lowly as possible
#[no_mangle]
unsafe extern "C" fn mbsr_get_rep_path(
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
        && extension != ".material.bin" {
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
        return log_dismiss("Dismissing path: {rs_filename:#?}");
    });
    result
}

#[inline(always)]
fn log_dismiss(reason: &str) -> Id<NSString>{
    log::warn!(target: "MBHook", "{reason}");
    //Really wanting to get rid of this, objc calls are expensive
    NSString::from_str("")
}
