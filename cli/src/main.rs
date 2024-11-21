use clap::Parser;
use clap::Subcommand;

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
    command: Commands,
    /// Verbosity level (0 = quite, 1 = standard, 2 = warning, 3 = error, 4 = info, 5 = verbose)
    #[arg(long, default_value_t = 1)]
    verbose: usize,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new project from a template
    Init {
        /// The path of where to store your project binaries
        #[arg(short, long = "~/.cardano-devkit")]
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

    let args = Args::parse();
    utils::print_header();
    logger::init(args.verbose);

    match args.command {
        Commands::Init { path } => {
            config::init(path);
            utils::check_setup().await.unwrap_or_else(|e| {
                logger::error(&format!(
                    "Failed to check your Yaci DevKit and services setup: {}",
                    e
                ));
                std::process::exit(1);
            });
        }
        Commands::Start => match start::start_devkit() {
            Ok(_) => logger::log("Cardano DevKit started successfully"),
            Err(e) => {
                logger::error(&format!("Failed to start Cardano DevKit: {}", e));
                std::process::exit(1);
            }
        },
        Commands::Stop => logger::log("Stop command not implemented yet"),
    }
}
