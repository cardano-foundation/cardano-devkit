use crate::logger::log;
use dirs::home_dir;
use fs_extra::dir::create_all;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub yaci_devkit: YaciDevkit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YaciDevkit {
    pub path: String,
    pub version: String,
    pub services_path: String,
}

impl Config {
    fn default(path: String) -> Self {
        Config {
            yaci_devkit: YaciDevkit {
                path: Path::new(&path)
                    .join("yaci-devkit")
                    .to_string_lossy()
                    .to_string(),
                services_path: Path::new(&path)
                    .join("services")
                    .to_string_lossy()
                    .to_string(),
                version: "0.9.3-beta".to_string(),
            },
        }
    }

    fn create_config_file() -> Self {
        let default_config = Config::default(get_devkit_root());

        let json_content =
            serde_json::to_string_pretty(&default_config).expect("Failed to serialize config.");
        let config_path = Path::new(&get_devkit_root())
            .join("config.json")
            .to_string_lossy()
            .to_string();
        if let Some(parent) = Path::new(&config_path).parent() {
            create_all(parent, true).expect("Failed to create config directory.");
        }
        fs::write(Path::new(&config_path), json_content).expect("Failed to write config file.");
        default_config
    }

    fn load() -> Self {
        let config_path = Path::new(&get_devkit_root())
            .join("config.json")
            .to_string_lossy()
            .to_string();
        if Path::new(&config_path).exists() {
            let file_content =
                fs::read_to_string(config_path).expect("Failed to read config file.");
            serde_json::from_str(&file_content).unwrap_or_else(|_| {
                eprintln!("Failed to parse config file, using default config.");
                Config::default(get_devkit_root())
            })
        } else {
            Self::create_config_file()
        }
    }

    fn save_to_file(self) -> Self {
        let json_content =
            serde_json::to_string_pretty(&self).expect("Failed to serialize config.");
        let config_path = Path::new(&get_devkit_root())
            .join("config.json")
            .to_string_lossy()
            .to_string();
        fs::write(Path::new(&config_path), json_content).expect("Failed to write config file.");
        self
    }

    pub fn from_string(json: &str) -> Self {
        serde_json::from_str(json).expect("Failed to parse config.")
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Failed to serialize config.")
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default(get_devkit_root()));
}

#[tauri::command]
pub fn get_devkit_root() -> String {
    if let Some(home_path) = home_dir() {
        format!("{}/.cardano-devkit", home_path.as_path().display())
    } else {
        "/root/.cardano-devkit".to_string()
    }
}

#[tauri::command]
pub fn is_initialized() -> bool {
    let devkit_root = get_devkit_root();
    Path::new(&devkit_root).exists()
}

pub fn init() -> Config {
    if is_initialized() {
        log("Config already initialized. Skipping initialization.");
        get_config()
    } else {
        Config::create_config_file()
    }
}

pub fn load() {
    let mut config = CONFIG.lock().unwrap();
    *config = Config::load();
}

pub fn get_config() -> Config {
    Config::load()
}

pub fn update_from_string(json: &str) -> Config {
    Config::from_string(json).save_to_file()
}
