use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod repo;

#[derive(Parser)]
#[command(name = "rustygit")]
#[command(about = "A minimal reimplementation of git in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    println!("Hello, world!");
}
