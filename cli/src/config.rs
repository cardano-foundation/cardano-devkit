use crate::logger::{error, get_verbosity, log, verbose, Verbosity};
use crate::utils::{download_file, unzip_file, IndicatorMessage};
use console::style;
use dirs::home_dir;
use fs_extra::dir::create_all;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub yaci_devkit: YaciDevkit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YaciDevkit {
    pub path: String,
    pub version: String,
}

pub async fn create_config_file(config_path: &str) -> Config {
    let mut default_config = Config::default();

    if get_verbosity() == Verbosity::Verbose
        || get_verbosity() == Verbosity::Info
        || get_verbosity() == Verbosity::Standard
    {
        println!("Config file not found at: {}", config_path);
        let mut input = String::new();
        log(&format!(
            "Do you want to create it now? ({}es/no): ",
            style("y").bold().underlined()
        ));
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        if let Some(home_path) = home_dir() {
            if input.trim().eq_ignore_ascii_case("yes")
                || input.trim().eq_ignore_ascii_case("y")
                || input.trim().is_empty()
            {
                let default_project_root = format!(
                    "{}/.cardano-devkit/yaci-devkit",
                    home_path.as_path().display()
                );
                log(&format!(
                    "Please enter the path to 'yaci-devkit'. If it's not already installed, it will be downloaded automatically. (default: {}):",
                    default_project_root
                ));

                let mut project_root = String::new();
                stdin().read_line(&mut project_root).unwrap();
                let mut project_root = if project_root.trim().is_empty() {
                    default_project_root
                } else {
                    project_root.trim().to_string()
                };

                if project_root.starts_with("~") {
                    project_root = project_root.replace("~", home_path.to_str().unwrap());
                }
                let project_root_path = Path::new(&project_root);
                let parent_dir = project_root_path.parent().unwrap();

                if !project_root_path.exists() {
                    verbose(&format!(
                        "yaci-devkit folder does not exist. It will be downloaded to: {}",
                        project_root_path.display(),
                    ));
                    fs::create_dir_all(parent_dir).expect("Failed to create project root folder.");
                    let github_url = format!("https://github.com/bloxbean/yaci-devkit/releases/download/v{0}/yaci-devkit-{0}.zip", default_config.yaci_devkit.version);
                    download_file(
                        github_url.as_str(),
                        &parent_dir.join("yaci-devkit.zip"),
                        Some(IndicatorMessage {
                            message: "Downloading Yaci DevKit".to_string(),
                            step: "Step 1/2".to_string(),
                            emoji: "📥 ".to_string(),
                        }),
                    )
                    .await
                    .expect("Failed to download Yaci DevKit");

                    log(&format!(
                        "{} 📦 Extracting Yaci DevKit...",
                        style("Step 2/2").bold().dim()
                    ));

                    unzip_file(
                        parent_dir.join("yaci-devkit.zip").as_path(),
                        project_root_path,
                    )
                    .expect("Failed to unzip Yaci DevKit");
                    fs::remove_file(parent_dir.join("yaci-devkit.zip"))
                        .expect("Failed to cleanup yaci-devkit.zip");
                }

                default_config.yaci_devkit.path = project_root.clone();
                verbose(&format!(
                    "Yaci DevKit path set to: {}",
                    default_config.yaci_devkit.path
                ));
            } else {
                error("Config file not found. Exiting.");
                process::exit(0);
            }
        } else {
            error("Failed to resolve home directory. Exiting.");
            process::exit(0);
        }
    } else {
        error("No config file has been found. Creating a new config does not work with log levels warning, error or quite.");
        process::exit(0);
    }

    verbose(&format!(
        "Cardano DevKit config file: {:#?}",
        default_config
    ));

    default_config
}

impl Config {
    fn default() -> Self {
        let mut default_config = Config {
            yaci_devkit: YaciDevkit {
                path: "/root/.cardano-devkit/yaci-devkit".to_string(),
                version: "0.9.3-beta".to_string(),
            },
        };

        if let Some(home_path) = home_dir() {
            let default_project_root = format!(
                "{}/.cardano-devkit/yaci-devkit",
                home_path.as_path().display()
            );
            default_config.yaci_devkit.path = default_project_root.clone();
        }
        default_config
    }

    async fn load_from_file(config_path: &str) -> Self {
        if Path::new(config_path).exists() {
            let file_content =
                fs::read_to_string(config_path).expect("Failed to read config file.");
            serde_json::from_str(&file_content).unwrap_or_else(|_| {
                eprintln!("Failed to parse config file, using default config.");
                Config::default()
            })
        } else {
            let default_config = create_config_file(config_path).await;
            let parent_dir = Path::new(config_path).parent().unwrap();
            create_all(parent_dir, false).expect("Failed to create config dir.");
            let json_content = serde_json::to_string_pretty(&default_config)
                .expect("Failed to serialize default config.");
            fs::write(Path::new(config_path), json_content)
                .expect("Failed to write default config file.");
            default_config
        }
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

pub async fn init(config_path: &str) {
    let mut config = CONFIG.lock().unwrap();
    *config = Config::load_from_file(config_path).await;
}

#[allow(dead_code)]
pub fn get_config() -> Config {
    CONFIG.lock().unwrap().clone()
}
