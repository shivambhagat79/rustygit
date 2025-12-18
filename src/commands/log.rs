use crate::utils;
use anyhow::{Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_current_commit_hash(root_path: &Path) -> Result<Option<String>> {
    let head_path = root_path.join(".rustygit").join("HEAD");
    let head_content = std::fs::read_to_string(head_path)?.trim().to_string();

    let ref_path = if head_content.starts_with("ref: ") {
        Some(head_content[5..].trim())
    } else {
        return Ok(Some(head_content));
    };

    let ref_file_path: PathBuf;

    match ref_path {
        Some(ref_path) => {
            ref_file_path = root_path.join(".rustygit").join(ref_path);
            if !ref_file_path.exists() {
                return Ok(None);
            }
        }
        None => {
            bail!("HEAD couldn not be resolved.");
        }
    }

    let current_commit_hash = std::fs::read_to_string(ref_file_path)?.trim().to_string();

    Ok(Some(current_commit_hash))
}

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

    let commit_hash = get_current_commit_hash(root_path)?;

    let mut commit_hash = match commit_hash {
        Some(hash) => {
            if hash.is_empty() {
                println!("No commits found in the repository.");
                return Ok(());
            }
            hash
        }
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
