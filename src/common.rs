use crate::mc_utils::DataManager;
use crate::SHADER_PATHS;
use notify::event::{AccessKind, AccessMode, EventKind};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

pub(crate) fn setup_json_watcher<T: AsRef<Path>>(app_dir: T) -> Result<(), notify::Error> {
    let path: &Path = app_dir.as_ref();
    let mut dataman = DataManager::init_data(path);
    let (sender, reciever) = crossbeam_channel::unbounded();
    let mut watcher = RecommendedWatcher::new(sender, Config::default())?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    for event in reciever {
        let event = match event {
            Ok(event) => event,
            Err(e) => {
                log::info!("Skipping event error: {e}");
                continue;
            }
        };
        if event.kind != EventKind::Access(AccessKind::Close(AccessMode::Write)) {
            log::info!("Skipping event..");
            continue;
        }
        log::info!("Recieved interesting event: {:#?}", event);
        debug_assert!(!event.paths.is_empty());
        let Some(file_name) = event.paths.first().and_then(|p| p.file_name()) else {
            log::warn!("Event path is not a filename");
            continue;
        };
        if file_name == "global_resource_packs.json" {
            log::info!("Grp changed, updating..");
            update_global_sp(&mut dataman, false);
        }
        if file_name == "valid_known_packs.json" {
            log::info!("Vpk changed, full updating..");
            update_global_sp(&mut dataman, true);
        }
    }
    unreachable!("The loop should never break");
    Ok(())
}
fn update_global_sp(dataman: &mut DataManager, full: bool) {
    // it being poisoned is impossible
    let mut locked_sp = SHADER_PATHS.lock().unwrap();
    if full {
        if let Err(e) = dataman.update_validpacks() {
            log::warn!("Cant update valid packs: {e:#?}");
            return;
        };
    }
    let data = match dataman.shader_paths() {
        Ok(spaths) => spaths,
        Err(e) => {
            log::warn!("Cant update shader paths: {e:#?}");
            return;
        }
    };
    *locked_sp = data;
    log::info!("Updated global shader paths: {:#?}", &locked_sp);
}
