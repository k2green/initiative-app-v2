use std::sync::LockResult;

use log::Level;

pub enum Error {
    IoError(std::io::Error),
    TomlDeserializeError(toml::de::Error),
    TomlSerializeError(toml::ser::Error)
}

pub fn log_lock_error<T>(result: LockResult<T>, msg: impl Into<String>) -> LockResult<T> {
    match result {
        Ok(res) => Ok(res),
        Err(err) => {
            log::warn!("{}: {}", msg.into(), err.to_string());
            Err(err)
        }
    }
}

pub fn log<T: std::fmt::Display>(error: T, level: Level) -> T {
    match level {
        Level::Error => log::error!("{}", error),
        Level::Warn => log::warn!("{}", error),
        Level::Info => log::info!("{}", error),
        Level::Debug => log::debug!("{}", error),
        Level::Trace => log::trace!("{}", error),
    };
    error
}