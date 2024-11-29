use std::path::Path;
use std::process::Command;

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;
use logger::error;
use logger::log;

mod config;
mod logger;
mod start;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    command: Option<Commands>,
    /// Verbosity level (0 = quite, 1 = standard, 2 = warning, 3 = error, 4 = info, 5 = verbose)
    #[arg(long, default_value_t = 1)]
    verbose: usize,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new project from a template
    Init {
        /// The path of where to store your project binaries. Default is ~/.cardano-devkit
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Starts a local cardano development environment including all necessary components
    Start,
    /// Stops the local cardano development environment
    Stop,
}

#[tokio::main]
async fn main() {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    if !(os == "linux" && arch == "x86") && !(os == "macos" && arch == "aarch64") {
        eprintln!(
            "Unfortunately, your operating system ({}, {}) is not currently supported. Please feel free to submit a feature request at: https://github.com/cardano-foundation/cardano-devkit/issues/new/choose",
            os, arch
        );
        std::process::exit(1);
    }

    let parsed_args = Args::try_parse();
    utils::print_header();
    logger::init(parsed_args.as_ref().map(|args| args.verbose).unwrap_or(1));
    config::init();

    utils::check_setup().await.unwrap_or_else(|e| {
        logger::error(&format!(
            "Failed to check your Yaci DevKit and services setup: {}",
            e
        ));
        std::process::exit(1);
    });

    match parsed_args {
        Ok(args) => match args.command {
            Some(Commands::Init) { path } => {
                config::init(path);
                utils::check_setup().await.unwrap_or_else(|e| {
                    logger::error(&format!(
                        "Failed to check your Yaci DevKit and services setup: {}",
                        e
                    ));
                    std::process::exit(1);
                });
            }
            Some(Commands::Start) => match start::start_devkit() {
                Ok(_) => logger::log("Cardano DevKit started successfully"),
                Err(e) => {
                    logger::error(&format!("Failed to start Cardano DevKit: {}", e));
                    std::process::exit(1);
                }
            },
            Some(Commands::Stop) => logger::log("Stop command not implemented yet"),
            None => {
                error("Please provide a subcommand to execute.");
                log(&format!(
                    "{}",
                    Args::command().render_long_help().to_string()
                ));
                std::process::exit(1);
            }
        },
        Err(_) => {
            let cli_args: Vec<String> = std::env::args().skip(1).collect();
            let configuration = config::get_config();
            let yaci_devkit_path = Path::new(&configuration.yaci_devkit.path);

            let output = Command::new(yaci_devkit_path.join("yaci-cli"))
                .current_dir(yaci_devkit_path)
                .args(cli_args)
                .output()
                .map_err(|yaci_cli_error| {
                    error(&format!(
                        "Failed to execute {}/yaci-cli: {}",
                        yaci_devkit_path.display(),
                        yaci_cli_error
                    ))
                })
                .expect("Failed to execute yaci-cli");

            if output.status.success() {
                log(&String::from_utf8_lossy(&output.stdout));
            } else {
                error(&String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
    }
}
