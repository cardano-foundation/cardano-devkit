use clap::Parser;
use clap::Subcommand;

mod logger;
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
    Init,
    /// Starts a local cardano development environment including all necessary components
    Start,
    /// Stops the local cardano development environment
    Stop,
}

fn main() {
    let args = Args::parse();
    utils::print_header();
    logger::init(args.verbose);

    match args.command {
        Commands::Init => logger::log("Init command not implemented yet"),
        Commands::Start => logger::log("Start command not implemented yet"),
        Commands::Stop => logger::log("Stop command not implemented yet"),
    }
}
