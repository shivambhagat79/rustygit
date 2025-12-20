use crate::utils;
use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

fn get_current_branch_name(root_path: &Path) -> Result<Option<String>> {
    let head_path = root_path.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(head_path)?.trim().to_string();

    if head_content.starts_with("ref: ") {
        let ref_path = head_content[5..].trim();
        if ref_path.starts_with("refs/heads/") {
            let branch_name = &ref_path["refs/heads/".len()..];
            return Ok(Some(branch_name.to_string()));
        }
    }

    Ok(None)
}

pub fn create_branch(root_path: &Path, branch_name: &str) -> Result<()> {
    utils::ensure_repo_exists(root_path)?;

    let refs_path = root_path.join(".rustygit").join("refs").join("heads");

    if refs_path.join(branch_name).exists() {
        bail!("Branch '{}' already exists.", branch_name);
    }

    let current_commit_hash = utils::get_current_commit_hash(root_path)?.unwrap_or_default();

    fs::write(refs_path.join(branch_name), &current_commit_hash)?;

    Ok(())
}

pub fn branch(root_path: &Path) -> Result<()> {
    let current_branch = get_current_branch_name(root_path)?;
    let refs_path = root_path.join(".rustygit").join("refs").join("heads");
    let dir = fs::read_dir(refs_path)?;

    if let Some(current) = current_branch {
        for entry in dir {
            let entry = entry?;
            let branch_name = entry.file_name().into_string().unwrap();
            if branch_name == current {
                println!("* {}", branch_name);
            } else {
                println!("  {}", branch_name);
            }
        }
    } else {
        for entry in dir {
            let entry = entry?;
            let branch_name = entry.file_name().into_string().unwrap();
            println!("  {}", branch_name);
        }
    }
    Ok(())
}
