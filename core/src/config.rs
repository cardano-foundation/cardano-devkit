use crate::logger::log;
use dirs::home_dir;
use fs_extra::dir::create_all;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::Path;
use std::process::exit;
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

    fn load(path: Option<String>) -> Self {
        let config_path = Path::new(&get_devkit_root())
            .join("config.json")
            .to_string_lossy()
            .to_string();
        if Path::new(&config_path).exists() {
            let file_content =
                fs::read_to_string(config_path).expect("Failed to read config file.");
            serde_json::from_str(&file_content).unwrap_or_else(|_| {
                eprintln!("Failed to parse config file, using default config.");
                Config::default(path.unwrap_or(get_devkit_root()))
            })
        } else {
            let default_config = Config::default(path.unwrap_or(get_devkit_root()));
            log(&format!("🚀 Looks like it's your first time using the Cardano DevKit. Let's set up a config for you at: {}", config_path));

            let parent_dir = Path::new(&config_path).parent().unwrap();
            create_all(parent_dir, false).expect("Failed to create config dir.");
            let json_content = serde_json::to_string_pretty(&default_config)
                .expect("Failed to serialize default config.");
            fs::write(Path::new(&config_path), json_content)
                .expect("Failed to write default config file.");

            log(&format!(
                "✅ The Cardano DevKit config file has been created successfully! Please review its contents, and if you're happy with it, run cardano-devkit again to initialize its components: {:#?}",
                default_config
            ));
            log(
                "💡 Hint: The services directory will take up a few hundred megabytes since it will contain the cardano-node, yaci-store, and other services. You can change its path if you prefer not to store it in your home folder."
            );
            exit(0);
        }
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default(get_devkit_root()));
}

pub fn get_devkit_root() -> String {
    if let Some(home_path) = home_dir() {
        format!("{}/.cardano-devkit", home_path.as_path().display())
    } else {
        "/root/.cardano-devkit".to_string()
    }
}

pub fn init(path: Option<String>) {
    let mut config = CONFIG.lock().unwrap();
    *config = Config::load(path);
}

pub fn get_config() -> Config {
    CONFIG.lock().unwrap().clone()
}
