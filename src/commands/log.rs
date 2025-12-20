use crate::utils;
use anyhow::Result;
use std::{fs, path::Path};

fn get_parent_commit_hash(commit_data: &str) -> Option<String> {
    for lines in commit_data.lines() {
        if lines.starts_with("parent ") {
            let parent_hash = lines[7..].trim().to_string();
            return Some(parent_hash);
        }
    }
    None
}

pub fn log(root_path: &Path) -> Result<()> {
    utils::ensure_repo_exists(root_path)?;

    let commit_hash = utils::get_current_commit_hash(root_path)?;

    let mut commit_hash: String = match commit_hash {
        Some(hash) => hash,
        None => {
            println!("No commits found in the repository.");
            return Ok(());
        }
    };

    println!("Rusty Git Commit history:\n");

    while !commit_hash.is_empty() {
        let commit_path = root_path
            .join(".rustygit/objects")
            .join(&commit_hash[..2])
            .join(&commit_hash[2..]);
        let commit_data = fs::read_to_string(&commit_path)?;

        let formatted = utils::format_commit_history(&commit_data, &commit_hash)?;
        println!("{}", formatted);

        commit_hash = get_parent_commit_hash(&commit_data).unwrap_or_default();
    }

    Ok(())
}
