use std::{path::{PathBuf, Path}, fs, sync::{Mutex, Arc}, thread};

use serde::{Deserialize, Serialize, Deserializer};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    current_path: PathBuf,
    #[serde(skip_serializing, deserialize_with="deserialize_mutex")]
    is_saving: Arc<Mutex<bool>>
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            current_path: get_home_dir(),
            is_saving: Arc::new(Mutex::new(false))
        }
    }
}

impl AppConfig {
    pub fn load_from(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(Error::IoError)?;
        toml::from_str(&content).map_err(Error::TomlDeserializeError)
    }

    pub fn load_from_or_default(path: &Path) -> Self {
        match Self::load_from(path) {
            Ok(config) => config,
            Err(_) => Self::default()
        }
    }

    pub fn load_from_default_path() -> Self {
        Self::load_from_or_default(&get_config_path())
    }

    pub fn save_to(&self, path: &Path) -> Result<(), Error> {
        let content = toml::to_string_pretty(&self).map_err(Error::TomlSerializeError)?;
        fs::write(path, content).map_err(Error::IoError)
    }

    pub fn save_to_default_path(&self) -> Result<(), Error> {
        self.save_to(&get_config_path())
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn set_current_path(&mut self, path: impl Into<PathBuf>) {
        self.current_path = path.into();
    }

    fn is_saving(&self) -> bool {
        match self.is_saving.lock() {
            Ok(guard) => *guard,
            Err(_) => true
        }
    }

    fn save_async(&self) {
        if !self.is_saving() {
            let is_saving = self.is_saving.clone();
            let config = self.clone();
            thread::spawn(move || {
                config.save_to_default_path();
            });
        }
    }
}

pub fn get_home_dir() -> PathBuf {
    dirs::data_dir().unwrap_or(PathBuf::from("/home"))
}

pub fn get_data_dir() -> PathBuf {
    dirs::data_dir().unwrap_or(PathBuf::from("/data"))
}

pub fn get_app_data_dir() -> PathBuf {
    get_data_dir().join("InitiativeApp")
}

fn get_config_path() -> PathBuf {
    get_app_data_dir().join("config.toml")
}

fn deserialize_mutex<'de, D>(_: D) -> Result<Arc<Mutex<bool>>, D::Error>
where D: Deserializer<'de> {
    Ok(Arc::new(Mutex::new(false)))
}