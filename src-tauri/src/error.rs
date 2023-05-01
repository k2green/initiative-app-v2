use std::sync::LockResult;

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