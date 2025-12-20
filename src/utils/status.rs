use crate::{commands, utils};
use anyhow::{Result, bail};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub fn get_current_commit_hash(root_path: &Path) -> Result<Option<String>> {
    let head_path = root_path.join(".rustygit").join("HEAD");
    let head_content = std::fs::read_to_string(head_path)?.trim().to_string();

    let ref_path = if head_content.starts_with("ref: ") {
        head_content[5..].trim()
    } else {
        return Ok(Some(head_content));
    };

    let ref_file_path = root_path.join(".rustygit").join(ref_path);
    if !ref_file_path.exists() {
        bail!("HEAD could not be resolved.");
    }

    let current_commit_hash = fs::read_to_string(ref_file_path)?.trim().to_string();

    if current_commit_hash.is_empty() {
        return Ok(None);
    }

    Ok(Some(current_commit_hash))
}

pub fn get_current_tree_hash(root_path: &Path) -> Result<Option<String>> {
    let current_commit_hash = get_current_commit_hash(root_path)?;

    if let Some(commit_hash) = current_commit_hash {
        let commit_content = utils::parse_commit(root_path, &commit_hash)?;

        let n_idx = commit_content.find('\n').ok_or_else(|| {
            anyhow::anyhow!("Malformed commit object: missing newline after tree")
        })?;

        let tree_hash = commit_content[5..n_idx].to_string();

        Ok(Some(tree_hash))
    } else {
        Ok(None)
    }
}

pub fn get_tree_files_map(
    root_path: &Path,
    path: &Path,
    tree_hash: &str,
    files_map: &mut HashMap<PathBuf, String>,
) -> Result<()> {
    let tree_entries = utils::parse_tree(root_path, tree_hash)?;

    for entry in tree_entries {
        let entry_path = path.join(&entry.name);

        if entry.mode == "40000" {
            get_tree_files_map(
                root_path,
                &entry_path,
                &utils::bytes_to_hex(&entry.hash),
                files_map,
            )?;
        } else {
            files_map.insert(entry_path, utils::bytes_to_hex(&entry.hash));
        }
    }

    Ok(())
}

pub fn get_work_dir_map(
    root_path: &Path,
    path: &Path,
    map: &mut HashMap<PathBuf, String>,
) -> Result<()> {
    let dir = fs::read_dir(root_path.join(path))?;

    for entry in dir {
        let entry = entry?;
        let entry_path = path.join(entry.file_name());

        if entry.file_name() == ".rustygit" || entry.file_name() == ".git" {
            continue;
        }

        if entry.file_type()?.is_dir() {
            get_work_dir_map(&root_path, &entry_path, map)?;
        } else {
            let file_content = fs::read(root_path.join(&entry_path))?;
            let blob_content = commands::format_object(&file_content);
            let blob_hash = utils::hash_bytes(&blob_content);

            map.insert(entry_path, blob_hash);
        }
    }
    Ok(())
}
