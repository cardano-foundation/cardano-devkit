use console::style;
use dirs::home_dir;
use indicatif::ProgressBar;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, process};
use tokio::io::AsyncWriteExt;
use zip::read::ZipArchive;

use crate::config;
use crate::logger::{error, log};

pub fn print_header() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!(
        r#"
   _____              _                    
  / ____|            | |                   
 | |     __ _ _ __ __| | __ _ _ __   ___   
 | |    / _` | '__/ _` |/ _` | '_ \ / _ \  
 | |___| (_| | | | (_| | (_| | | | | (_) | 
  \_____\__,_|_|  \__,_|\__,_|_| |_|\___/  
  _____             _  ___ _               
 |  __ \           | |/ (_) |              
 | |  | | _____   _| ' / _| |_             
 | |  | |/ _ \ \ / /  < | | __|            
 | |__| |  __/\ V /| . \| | |_             
 |_____/ \___| \_/ |_|\_\_|\__| v{}    
    "#,
        VERSION
    );
}

pub struct IndicatorMessage {
    pub message: String,
    pub step: String,
    pub emoji: String,
}

pub async fn download_file(
    url: &str,
    path: &Path,
    indicator_message: Option<IndicatorMessage>,
) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::get(url).await?.error_for_status()?;

    let total_size = response.content_length();
    let mut fallback_message = String::from("Downloading ...");

    if let Some(indicator_message) = indicator_message {
        println!(
            "{} {}{}",
            style(indicator_message.step).bold().dim(),
            indicator_message.emoji,
            indicator_message.message
        );
        fallback_message = indicator_message.message;
    }

    let progress_bar = match total_size {
        Some(size) => ProgressBar::new(size),
        None => ProgressBar::new_spinner().with_message(fallback_message),
    };

    let mut file = tokio::fs::File::create(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        progress_bar.inc(chunk.len() as u64);
    }

    progress_bar.finish_with_message(format!("Downloaded {} to {}", url, path.to_string_lossy()));
    return Ok(());
}

pub fn unzip_file(file_path: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Open the ZIP file
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    let file_count = archive.len();
    let progress_bar = ProgressBar::new(file_count as u64);

    let mut root_folder: Option<PathBuf> = None;

    for i in 0..file_count {
        let mut file = archive.by_index(i)?;
        let outpath = destination.join(file.name());

        if i == 1 {
            if let Some(parent) = outpath.parent() {
                root_folder = Some(parent.to_path_buf());
            }
        }

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }

        progress_bar.set_message(file.name().to_string());
        progress_bar.inc(1);
    }

    if let Some(root_folder) = root_folder {
        if root_folder != *destination {
            for entry in fs::read_dir(&root_folder)? {
                let entry = entry?;
                let path = entry.path();
                let file_name = path.file_name().unwrap(); // safe unwrap
                let new_path = destination.join(file_name);
                fs::rename(path, new_path)?;
            }
            fs::remove_dir_all(root_folder)?;
        }
    }

    Ok(())
}

pub fn resolve_home_symbol(path: &str) -> String {
    if path.contains("~") {
        if let Some(home_path) = home_dir() {
            let home_path = home_path.to_str().unwrap();
            return path.replace("~", home_path);
        } else {
            return path.replace("~", "/root");
        }
    }
    path.to_string()
}

pub fn default_config_path() -> PathBuf {
    let mut config_path = home_dir().unwrap_or_else(|| PathBuf::from("~"));
    config_path.push(".cardano-devkit");
    config_path.push("config.json");
    config_path
}

pub async fn check_setup() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();
    let yaci_devkit_root = resolve_home_symbol(&config.yaci_devkit.path);
    let servcies_root = resolve_home_symbol(&config.yaci_devkit.services_path);

    let yaci_devkit_path = Path::new(&yaci_devkit_root);
    let services_path = Path::new(&servcies_root);

    if !yaci_devkit_path.exists() {
        download_and_configure_yaci_devkit(
            yaci_devkit_path,
            services_path,
            &config.yaci_devkit.version,
        )
        .await?;
        download_services(yaci_devkit_path)?;
    }

    Ok(())
}

fn set_executable_permission(file_path: &Path) -> std::io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    let mut permissions = metadata.permissions();
    let mode = permissions.mode() | 0o111; // Add execute permission
    permissions.set_mode(mode);
    fs::set_permissions(file_path, permissions)?;
    Ok(())
}

pub fn download_services(yaci_devkit_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let yaci_cli_path = yaci_devkit_path.join("yaci-cli");
    set_executable_permission(&yaci_cli_path.as_path())?;

    Command::new(yaci_cli_path)
        .current_dir(yaci_devkit_path)
        .arg("download")
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute {}/yaci-cli download: {}",
                yaci_devkit_path.display(),
                e
            )
        })?;
    Ok(())
}

pub async fn download_and_configure_yaci_devkit(
    yaci_devkit_path: &Path,
    services_path: &Path,
    yaci_devkit_version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaci_devkit_path_path = Path::new(&yaci_devkit_path);
    let parent_dir = yaci_devkit_path_path.parent().unwrap();

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).expect("Failed to create project root folder.");
    }

    if !services_path.exists() {
        fs::create_dir_all(services_path).expect("Failed to create services folder.");
    }

    let os = std::env::consts::OS;
    let github_url = match os {
        "linux" => format!("https://github.com/bloxbean/yaci-devkit/releases/download/v{0}/yaci-cli-{0}-linux-X64.zip", yaci_devkit_version),
        "macos" => format!("https://github.com/bloxbean/yaci-devkit/releases/download/v{0}/yaci-cli-{0}-macos-ARM64.zip", yaci_devkit_version),
        _ => {
            error(&format!("Unsupported OS: {}", os));
            process::exit(0);
        }
    };
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
        yaci_devkit_path_path,
    )
    .expect("Failed to unzip Yaci DevKit");

    fs::remove_file(parent_dir.join("yaci-devkit.zip")).expect("Failed to cleanup yaci-devkit.zip");

    let download_properties_path = yaci_devkit_path_path.join("config/download.properties");
    let mut download_properties_file = OpenOptions::new()
        .append(true)
        .open(download_properties_path)
        .expect("Failed to open download.properties");

    let yaci_cli_home = format!("\nyaci.cli.home={}\n", services_path.display());
    download_properties_file.write_all(yaci_cli_home.as_bytes())?;

    Ok(())
}
