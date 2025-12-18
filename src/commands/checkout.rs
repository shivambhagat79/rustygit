use crate::{commands::TreeEntry, utils};
use anyhow::{Result, anyhow};
use std::{fs, path::Path};

fn is_branch() -> Result<bool> {
    Ok(false)
}

fn clear_repository(root_path: &Path) -> Result<()> {
    let dir = fs::read_dir(root_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // skip rustygit.exe on Windows
            #[cfg(target_os = "windows")]
            {
                if let Some(file_name) = path.file_name() {
                    if file_name == "rustygit.exe" {
                        continue;
                    }
                }
            }
            // skip rustygit on Unix-like systems
            #[cfg(not(target_os = "windows"))]
            {
                if let Some(file_name) = path.file_name() {
                    if file_name == "rustygit.exe" {
                        continue;
                    }
                }
            }
            fs::remove_file(path)?;
        } else if path.is_dir() {
            if path.ends_with(".rustygit") || path.ends_with(".git") {
                continue;
            }
            fs::remove_dir_all(path)?;
        }
    }

    Ok(())
}

fn restore_tree(root_path: &Path, path: &Path, tree_hash: &str) -> Result<()> {
    let tree_entries: Vec<TreeEntry> = utils::parse_tree(root_path, tree_hash)?;

    for entry in tree_entries {
        let entry_path = path.join(&entry.name);

        if entry.mode == "40000" {
            fs::create_dir_all(&entry_path)?;
            let subtree_hash = utils::bytes_to_hex(&entry.hash);
            restore_tree(root_path, &entry_path, &subtree_hash)?;
        } else if entry.mode == "100644" {
            let blob_hash = utils::bytes_to_hex(&entry.hash);
            let blob_content = utils::parse_blob(root_path, &blob_hash)?;
            fs::write(&entry_path, blob_content)?;
        }
    }

    Ok(())
}

fn checkout_hash(root_path: &Path, target: &str) -> Result<()> {
    let commit_content = utils::parse_commit(root_path, target)?;

    let n_idx = commit_content
        .find('\n')
        .ok_or_else(|| anyhow!("Malformed commit object: missing newline after tree"))?;

    let tree_hash = &commit_content[5..n_idx];

    clear_repository(root_path)?;

    restore_tree(root_path, root_path, tree_hash)?;

    let head_path = root_path.join(".rustygit").join("HEAD");
    fs::write(head_path, target)?;

    Ok(())
}

pub fn checkout(root_path: &Path, target: &str) -> Result<()> {
    if is_branch()? {
        // To be implemented
    } else {
        checkout_hash(root_path, target)?;
    }
    Ok(())
}
