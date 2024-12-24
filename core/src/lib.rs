use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod config;
mod logger;
mod utils;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init() -> String {
    config::init();
    let config = config::get_config();
    serde_json::to_string_pretty(&config).unwrap()
}

#[tauri::command]
fn get_config() -> String {
    let config = config::get_config();
    serde_json::to_string_pretty(&config).unwrap()
}

#[tauri::command]
fn save_config(config: &str) -> String {
    config::update_from_string(config).to_string()
}

#[tauri::command]
async fn download_binaries() {
    utils::check_setup().await.unwrap_or_else(|e| {
        logger::error(&format!(
            "Failed to check your Yaci DevKit and services setup: {}",
            e
        ));
    });
}

#[tauri::command]
fn is_yaci_devkit_initialized() -> bool {
    let config = config::get_config();
    let yaci_devkit_path = Path::new(&config.yaci_devkit.path);
    if yaci_devkit_path.is_dir() {
        if let Ok(mut entries) = fs::read_dir(yaci_devkit_path) {
            return entries.next().is_some();
        }
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_tauri() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            config::is_initialized,
            config::get_devkit_root,
            init,
            get_config,
            save_config,
            download_binaries,
            is_yaci_devkit_initialized
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
