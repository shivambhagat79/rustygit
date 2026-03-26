use crate::utils::IgnoreRule;
use crate::{commands, utils};
use anyhow::Result;
use std::{
    collections::{BTreeSet, HashMap},
    path::{Path, PathBuf},
};

pub struct TreeEntry {
    pub(crate) mode: &'static str,
    pub(crate) name: String,
    pub(crate) hash: [u8; 20],
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

fn write_tree_from_index(
    repo_root: &Path,
    index_map: &HashMap<PathBuf, String>,
    prefix: &Path,
) -> Result<String> {
    let mut entries: Vec<TreeEntry> = Vec::new();
    let mut child_dirs: BTreeSet<String> = BTreeSet::new();

    for (path, hash) in index_map {
        let Ok(relative) = path.strip_prefix(prefix) else {
            continue;
        };

        let mut components = relative.components();
        let Some(first_component) = components.next() else {
            continue;
        };

        let first_name = first_component.as_os_str().to_string_lossy().to_string();

        if components.next().is_none() {
            entries.push(TreeEntry {
                mode: "100644",
                name: first_name,
                hash: utils::hex_to_bytes(hash),
            });
        } else {
            child_dirs.insert(first_name);
        }
    }

    for directory in child_dirs {
        let child_prefix = if prefix.as_os_str().is_empty() {
            PathBuf::from(&directory)
        } else {
            prefix.join(&directory)
        };

        let tree_hash = write_tree_from_index(repo_root, index_map, &child_prefix)?;
        entries.push(TreeEntry {
            mode: "40000",
            name: directory,
            hash: utils::hex_to_bytes(&tree_hash),
        });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    let tree_bytes = format_tree(&entries);
    let tree_hash = utils::hash_bytes(&tree_bytes);
    commands::write_object(repo_root, &tree_hash, &tree_bytes)?;

    Ok(tree_hash)
}

pub fn write_tree(repo_root: &Path, path: &Path, ignore_rules: &Vec<IgnoreRule>) -> Result<String> {
    let _ = path;
    let _ = ignore_rules;

    let index_map = utils::read_index_map(repo_root)?;
    write_tree_from_index(repo_root, &index_map, Path::new(""))
}
