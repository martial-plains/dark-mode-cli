use clap::{Parser, Subcommand};
use dark_mode::DarkMode;

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Toggle dark mode")]
    Toggle,
    #[clap(about = "Check dark mode status")]
    Status,
    #[clap(about = "Enable dark mode")]
    On,
    #[clap(about = "Disable dark mode")]
    Off,
}

fn main() {
    let mut dark_mode = DarkMode::new();

    // Check if any command is given
    if std::env::args().count() < 2 {
        dark_mode.toggle();
        return;
    }

    // Parse command
    let args = Cli::parse();

    match args.command {
        Commands::Toggle => dark_mode.toggle(),
        Commands::Status => println!("{}", dark_mode.status()),
        Commands::On => dark_mode.enable(),
        Commands::Off => dark_mode.disable(),
    }
}
