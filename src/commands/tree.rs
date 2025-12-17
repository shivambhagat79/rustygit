use crate::{commands, utils};
use anyhow::Result;
use std::fs;
use std::path::Path;

struct TreeEntry {
    mode: &'static str,
    name: String,
    hash: [u8; 20],
}

fn format_tree(entries: &[TreeEntry]) -> Vec<u8> {
    let mut data = Vec::new();

    for entry in entries {
        let header = format!("{} {}\0", entry.mode, entry.name);
        data.extend_from_slice(header.as_bytes());
        data.extend_from_slice(&entry.hash);
    }

    let header = format!("tree {}\0", data.len());
    let mut result = Vec::new();
    result.extend_from_slice(header.as_bytes());
    result.extend_from_slice(&data);
    result
}

pub fn write_tree(repo_root: &Path, path: &Path) -> Result<String> {
    let mut entries: Vec<TreeEntry> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry.file_name().into_string().unwrap();

        if name == ".rustygit" || name == ".git" {
            continue;
        }

        if entry_path.is_dir() {
            let tree_hash = write_tree(repo_root, &entry_path)?;
            entries.push(TreeEntry {
                mode: "40000",
                name,
                hash: utils::hex_to_bytes(&tree_hash),
            });
        } else if entry_path.is_file() {
            let blob_hash = commands::write_blob(repo_root, &entry_path)?;
            entries.push(TreeEntry {
                mode: "100644",
                name,
                hash: utils::hex_to_bytes(&blob_hash),
            });
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    let tree_bytes = format_tree(&entries);
    let tree_hash = utils::hash_bytes(&tree_bytes);

    commands::write_object(repo_root, &tree_hash, &tree_bytes)?;

    Ok(tree_hash)
}
