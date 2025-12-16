use crate::utils;
use anyhow::{Result, bail};
use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;

// ============================================================================
// Object Types
// ============================================================================

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Blob,
    Tree,
}

impl ObjectType {
    fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
        }
    }
}

// ============================================================================
// Low-level helpers (pure, reusable)
// ============================================================================

fn format_object(object_type: ObjectType, contents: &[u8]) -> Vec<u8> {
    let header = format!("{} {}\0", object_type.as_str(), contents.len());
    let mut result = Vec::with_capacity(header.len() + contents.len());
    result.extend_from_slice(header.as_bytes());
    result.extend_from_slice(contents);
    result
}

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn write_object(repo_root: &Path, hash: &str, data: &[u8]) -> Result<()> {
    let objects_dir = repo_root.join(".rustygit").join("objects");

    let (dir, file) = hash.split_at(2);
    let object_dir = objects_dir.join(dir);
    let object_path = object_dir.join(file);

    if object_path.exists() {
        return Ok(()); // Git behavior: objects are immutable
    }

    fs::create_dir_all(&object_dir)?;
    fs::write(object_path, data)?;
    Ok(())
}

// ============================================================================
// Blob handling
// ============================================================================

fn write_blob(repo_root: &Path, file_path: &Path) -> Result<String> {
    if !file_path.is_file() {
        bail!("Not a regular file: {}", file_path.display());
    }

    let content = fs::read(file_path)?;
    let object_bytes = format_object(ObjectType::Blob, &content);
    let hash = hash_bytes(&object_bytes);

    write_object(repo_root, &hash, &object_bytes)?;
    Ok(hash)
}

pub fn hash_object(file_path: &Path) -> Result<String> {
    let repo_root = std::env::current_dir()?;
    write_blob(&repo_root, file_path)
}

// ============================================================================
// Tree handling
// ============================================================================

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

pub fn write_tree(path: &Path) -> Result<String> {
    let repo_root = std::env::current_dir()?;
    let mut entries: Vec<TreeEntry> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry.file_name().into_string().unwrap();

        if name == ".rustygit" || name == ".git" {
            continue;
        }

        if entry_path.is_dir() {
            let tree_hash = write_tree(&entry_path)?;
            entries.push(TreeEntry {
                mode: "40000",
                name,
                hash: utils::hex_to_bytes(&tree_hash),
            });
        } else if entry_path.is_file() {
            let blob_hash = write_blob(&repo_root, &entry_path)?;
            entries.push(TreeEntry {
                mode: "100644",
                name,
                hash: utils::hex_to_bytes(&blob_hash),
            });
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    let tree_bytes = format_tree(&entries);
    let tree_hash = hash_bytes(&tree_bytes);

    write_object(&repo_root, &tree_hash, &tree_bytes)?;

    Ok(tree_hash)
}
