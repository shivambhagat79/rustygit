use anyhow::Result;
use clap::{Parser, Subcommand};
use rustygit::repo;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustygit")]
#[command(about = "A simple Git implementation in Rust", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rusty Git repository
    ///
    /// This command creates a `.rustygit` directory and
    /// initializes the required metadata to track versions.
    Init {
        /// Path where the repository should be initialized.
        ///
        /// If no path is provided, the current directory is used.
        path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            repo::init(&target_path)?;
            println!("Initialised Empty Rusty Git Repository.");
        }
    }

    Ok(())
}
