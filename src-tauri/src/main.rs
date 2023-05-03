// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod config;
pub mod error;

use std::{sync::Mutex, path::PathBuf, fs::{self, DirEntry}, cmp::Ordering};

use chrono::Local;
use common_data_lib::{creatures::{CreatureContainer, Creature, OrderMode, ConflictGroup}, BackendError, ToBackendResult};
use config::get_app_data_dir;
use error::{log_lock_error, log};
use log::{SetLoggerError, LevelFilter, Level};
use log4rs::{append::{console::{ConsoleAppender, Target}, file::FileAppender}, encode::pattern::PatternEncoder, Config, config::{Appender, Root}, filter::threshold::ThresholdFilter};
use tauri::{State, Manager};
use uuid::Uuid;

const MAX_LOG_COUNT: usize = 10;

#[derive(Debug)]
struct AppState {
    creatures: Mutex<CreatureContainer>,
    conflicts: Mutex<Option<Vec<ConflictGroup>>>
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            creatures: Mutex::new(CreatureContainer::default()),
            conflicts: Mutex::new(None)
        }
    }
}

#[tauri::command]
fn get_creatures(state: State<AppState>) -> Result<Vec<Creature>, BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    creatures_guard.set_order_mode(OrderMode::Alphabetical);

    Ok(creatures_guard.cloned())
}

#[tauri::command]
fn add_creatures(state: State<AppState>, creatures: String) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    for name in creatures.lines().filter(|l| !l.is_empty()) {
        let creature = Creature::from(name);
        log::info!("Adding new creature: {}", creature);
        creatures_guard.insert(creature);
    }

    Ok(())
}

#[tauri::command]
fn remove_creature(state: State<AppState>, id: Uuid) -> Result<Creature, BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    let creature = creatures_guard.remove(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))?;

    log::info!("Removed creature: {}", creature);

    Ok(creature)
}

#[tauri::command]
fn set_creature_selected(state: State<AppState>, id: Uuid, selected: bool) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    let creature = creatures_guard.get_mut(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))?;
    creature.set_selected(selected);

    log::info!("Set creature {} selected state to {}", creature, selected);

    Ok(())
}

#[tauri::command]
fn set_creature_initiative(state: State<AppState>, id: Uuid, initiative: isize) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    let creature = creatures_guard.get_mut(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))?;
    creature.set_initiative(initiative);

    log::info!("Set creature {} initiative to {}", creature, initiative);

    Ok(())
}

#[tauri::command]
fn set_all_creatures_selected(state: State<AppState>, selected: bool) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    for creature in creatures_guard.iter_mut() {
        creature.set_selected(selected);
    }

    log::info!("Set all creatures selected state to {}", selected);

    Ok(())
}

#[tauri::command]
fn reset_all_initiatives(state: State<AppState>) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    for creature in creatures_guard.iter_mut() {
        creature.set_initiative(0);
        creature.set_sub_order(0);
    }

    log::info!("Reset initiative order");

    Ok(())
}

#[tauri::command]
fn save_encounter(state: State<AppState>, path: PathBuf) -> Result<(), BackendError> {
    let creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    creatures_guard.save_to(&path)?;

    log::info!("Saved encounter to: '{}'", path.to_string_lossy());

    Ok(())
}

#[tauri::command]
fn load_encounter(state: State<AppState>, path: PathBuf) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    let new_creatures = CreatureContainer::load_from(&path)?;
    *creatures_guard = new_creatures;

    log::info!("Loaded encounter from: '{}'", path.to_string_lossy());

    Ok(())
}

#[tauri::command]
fn new_encounter(state: State<AppState>) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    *creatures_guard = CreatureContainer::default();

    log::info!("Beginning a new encounter");

    Ok(())
}

#[tauri::command]
fn get_initiative_conflicts(state: State<AppState>, set_conflicts: bool) -> Result<Vec<ConflictGroup>, BackendError> {
    let creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    let mut conflicts_guard = log_lock_error(state.conflicts.lock(), "Unable to lock conflicts state").to_backend_result()?;
    
    if set_conflicts {
        *conflicts_guard = Some(creatures_guard.get_conflicts());
    }

    match &*conflicts_guard {
        None => return Err(log(BackendError::logic_error("Could not get conflicts because they have not been generated"), Level::Error)),
        Some(conflicts) => Ok(conflicts.clone())
    }
}

#[tauri::command]
fn move_initiative_conflict(state: State<AppState>, group_index: usize, move_index: usize, target_index: usize) -> Result<(), BackendError> {
    let mut conflicts_guard = log_lock_error(state.conflicts.lock(), "Unable to lock conflicts state").to_backend_result()?;
    let conflicts = match &mut *conflicts_guard {
        Some(conflicts) => conflicts,
        None => return Err(log(BackendError::logic_error("Could not move conflict because they have not been generated"), Level::Error))
    };
    
    if group_index >= conflicts.len() {
        return Err(log(BackendError::argument_error("group_index", format!("Group index {} is out of bounds", move_index)), Level::Error));
    }

    let group = &mut conflicts[group_index];

    let creatures = group.creatures_mut();
    if move_index >= creatures.len() {
        return Err(log(BackendError::argument_error("move_index", format!("Creature index {} is out of bounds", move_index)), Level::Error));
    }

    if target_index >= creatures.len() {
        return Err(log(BackendError::argument_error("target_index", format!("Creature index {} is out of bounds", target_index)), Level::Error));
    }

    if move_index != target_index {
        let move_creature = creatures.remove(move_index);
        creatures.insert(target_index, move_creature);
    }

    Ok(())
}

#[tauri::command]
fn finalize_initiative_order(state: State<AppState>) -> Result<(), BackendError> {
    let mut creatures_guard = log_lock_error(state.creatures.lock(), "Unable to lock creatures state").to_backend_result()?;
    let mut conflicts_guard = log_lock_error(state.conflicts.lock(), "Unable to lock conflicts state").to_backend_result()?;
    if let Some(conflicts) = &mut *conflicts_guard {
        for group in conflicts {
            group.finalize(&mut creatures_guard);
        }
    }

    creatures_guard.finalize();

    Ok(())
}

fn get_default_state() -> AppState {
    AppState::default()

    // Mutex::new(AppState {
    //     creatures: CreatureContainer::from(vec! [
    //         Creature::from("Test 1"),
    //         Creature::from("Test 2"),
    //         Creature::from("Test 3"),
    //         Creature::from("Test 4"),
    //         Creature::from("Test 5"),
    //     ])
    // })
}

fn main() -> Result<(), SetLoggerError> {
    configure_logger()?;
    cleanup_logs();

    log::info!("Starting app");

    tauri::Builder::default()
        .manage(get_default_state())
        .invoke_handler(tauri::generate_handler![
            get_creatures,
            add_creatures,
            remove_creature,
            set_creature_selected,
            set_creature_initiative,
            set_all_creatures_selected,
            reset_all_initiatives,
            get_initiative_conflicts,
            move_initiative_conflict,
            finalize_initiative_order,
            save_encounter,
            load_encounter,
            new_encounter
        ])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn get_current_timestamp() -> String {
    Local::now().format("%d-%m-%y %H-%M-%S").to_string()
}

fn cleanup_logs() {
    match fs::read_dir(get_backend_log_dir()) {
        Err(err) => log::warn!("Unable to read log directory: {}", err),
        Ok(files) => {
            let mut files = files.into_iter()
                .filter_map(|file| match file {
                    Ok(file) => Some(file),
                    Err(err) => {
                        log::warn!("Unable to read log file: {}", err);
                        None
                    }
                })
                .collect::<Vec<_>>();

            if files.len() > MAX_LOG_COUNT {
                files.sort_by(cmp_files);
            }

            while files.len() > MAX_LOG_COUNT {
                let file = files.pop().unwrap();
                match fs::remove_file(file.path()) {
                    Ok(_) => log::info!("Removed log file: {}", file.path().to_string_lossy()),
                    Err(err) => log::info!("unable to remove log file '{}': {}", file.path().to_string_lossy(), err),
                }
            }
        }
    } 
}

fn cmp_files(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_metadata = match a.metadata() {
        Ok(metadata) => metadata,
        Err(err) => {
            log::warn!("Could not get metadata: {}", err);
            return Ordering::Equal;
        }
    };

    let b_metadata = match b.metadata() {
        Ok(metadata) => metadata,
        Err(err) => {
            log::warn!("Could not get metadata: {}", err);
            return Ordering::Equal;
        }
    };

    let a_accessed = match a_metadata.accessed() {
        Ok(time) => time,
        Err(err) => {
            log::warn!("Could not get metadata access time: {}", err);
            return Ordering::Equal;
        }
    };

    let b_accessed = match b_metadata.accessed() {
        Ok(time) => time,
        Err(err) => {
            log::warn!("Could not get metadata access time: {}", err);
            return Ordering::Equal;
        }
    };

    b_accessed.cmp(&a_accessed)
}

fn configure_logger() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let path = get_backend_log_dir()
        .join(format!("{}.log", get_current_timestamp()));

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}][{l}] - {m}\n")))
        .build(path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();
    
    let _handle = log4rs::init_config(config)?;

    Ok(())
}

fn get_backend_log_dir() -> PathBuf {
    get_app_data_dir()
        .join("logs")
        .join("backend")
}