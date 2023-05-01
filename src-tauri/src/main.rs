// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod config;
pub mod error;

use std::{sync::Mutex, path::PathBuf, fs::{self, DirEntry}, cmp::Ordering};

use chrono::Local;
use common_data_lib::{creatures::{CreatureContainer, Creature, OrderMode}, BackendError, ToBackendResult};
use config::get_app_data_dir;
use error::log_lock_error;
use log::{SetLoggerError, LevelFilter};
use log4rs::{append::{console::{ConsoleAppender, Target}, file::FileAppender}, encode::pattern::PatternEncoder, Config, config::{Appender, Root}, filter::threshold::ThresholdFilter};
use tauri::{State, Manager};
use uuid::Uuid;

const MAX_LOG_COUNT: usize = 10;

#[derive(Debug, Default, Clone)]
struct AppState {
    creatures: CreatureContainer,
}

#[tauri::command]
fn get_creatures(state: State<Mutex<AppState>>) -> Result<Vec<Creature>, BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    guard.creatures.set_order_mode(OrderMode::Alphabetical);

    Ok(guard.creatures.cloned())
}

#[tauri::command]
fn add_creatures(state: State<Mutex<AppState>>, creatures: String) -> Result<(), BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    for name in creatures.lines().filter(|l| !l.is_empty()) {
        guard.creatures.insert(Creature::from(name.trim()));
    }

    Ok(())
}

#[tauri::command]
fn remove_creature(state: State<Mutex<AppState>>, id: Uuid) -> Result<Creature, BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    guard.creatures.remove(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))
}

#[tauri::command]
fn set_creature_selected(state: State<Mutex<AppState>>, id: Uuid, selected: bool) -> Result<(), BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    let creature = guard.creatures.get_mut(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))?;
    creature.set_selected(selected);

    Ok(())
}

#[tauri::command]
fn set_creature_initiative(state: State<Mutex<AppState>>, id: Uuid, initiative: isize) -> Result<(), BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    let creature = guard.creatures.get_mut(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))?;
    creature.set_initiative(initiative);

    Ok(())
}

#[tauri::command]
fn set_creature_sub_order(state: State<Mutex<AppState>>, id: Uuid, sub_order: isize) -> Result<(), BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    let creature = guard.creatures.get_mut(id).ok_or(BackendError::argument_error("id", format!("No creature with id '{}' exists", id)))?;
    creature.set_sub_order(sub_order);

    Ok(())
}

#[tauri::command]
fn set_all_creatures_selected(state: State<Mutex<AppState>>, selected: bool) -> Result<(), BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    for creature in guard.creatures.iter_mut() {
        creature.set_selected(selected);
    }

    Ok(())
}

#[tauri::command]
fn reset_all_initiatives(state: State<Mutex<AppState>>) -> Result<(), BackendError> {
    let mut guard = log_lock_error(state.lock(), "Unable to lock app state").to_backend_result()?;
    for creature in guard.creatures.iter_mut() {
        creature.set_initiative(0);
        creature.set_sub_order(0);
    }

    Ok(())
}

fn get_default_state() -> Mutex<AppState> {
    Mutex::new(AppState::default())

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
            set_creature_sub_order,
            set_all_creatures_selected,
            reset_all_initiatives
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