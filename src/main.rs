use anyhow::Result;
use clap::{Parser, Subcommand};
use rustygit::utils::IgnoreRule;
use rustygit::{commands, utils};
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
    /// Logs the commit history
    ///
    /// This command displays the commit history of the repository.
    Log,
    /// Checkout a specific commit or branch
    ///
    /// This command updates the working directory to match the specified commit or branch
    /// and moves the head to that commit.
    Checkout {
        /// The target commit hash or branch name to checkout.
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    let root_path = std::env::current_dir()?;

    match cli.command {
        Commands::Init { path } => {
            let target_path = path.unwrap_or(root_path);
            commands::init(&target_path)?;
        }
        Commands::HashObject { file } => {
            let hash = commands::hash_object(&file)?;
            println!("File hashed successfully\nHash: {}", hash);
        }
        Commands::WriteTree => {
            let ignore_rules: Vec<IgnoreRule> = utils::parse_ignore_file(&root_path)?;
            let hash = commands::write_tree(&root_path, &root_path, &ignore_rules)?;
            println!("Tree written successfully\nHash: {}", hash);
        }
        Commands::Commit { message } => {
            let message = message.unwrap_or_else(|| String::from(""));
            let ignore_rules: Vec<IgnoreRule> = utils::parse_ignore_file(&root_path)?;
            let commit_hash = commands::commit(&root_path, message, &ignore_rules);

            println!("Committed successfully!\nHash: {}", commit_hash?);
        }
        Commands::Log => {
            commands::log(&root_path)?;
        }
        Commands::Checkout { target } => {
            commands::checkout(&root_path, &target)?;
        }
    }

    Ok(())
}
