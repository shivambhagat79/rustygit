use anyhow::Result;
use clap::{Parser, Subcommand};
use rustygit::commands;
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
    /// Hash a file as a Git object
    ///
    /// This command computes the SHA-1 hash of a file
    /// and stores it in the Git object database.
    HashObject {
        /// This is the path to the file that you want to hash.
        file: PathBuf,
    },
    /// Write the current directory tree as a Git object
    ///
    /// This command creates a tree object representing the
    /// current state of the directory and stores it in the object database.
    WriteTree,
    /// Commit the current tree with a message
    ///
    /// This command creates a commit object that points to the current tree
    /// and includes a commit message.
    Commit {
        /// The commit message describing the changes.
        #[arg(short, long)]
        message: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            commands::init(&target_path)?;
            println!("Initialised Empty Rusty Git Repository.");
        }
        Commands::HashObject { file } => {
            let hash = commands::hash_object(&file)?;
            println!("{}", hash);
        }
        Commands::WriteTree => {
            let hash = commands::write_tree(&PathBuf::from("."), &PathBuf::from("."))?;
            println!("{}", hash);
        }
        Commands::Commit { message } => {
            let message = message.unwrap_or_else(|| String::from(""));
            let commit_hash = commands::commit(&PathBuf::from("."), message);

            println!("Committed Successfully!\nHash: {}", commit_hash?);
        }
    }

    Ok(())
}
