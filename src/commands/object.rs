use crate::utils;
use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

fn format_object(contents: &[u8]) -> Vec<u8> {
    let header = format!("blob {}\0", contents.len());
    let mut result = Vec::with_capacity(header.len() + contents.len());
    result.extend_from_slice(header.as_bytes());
    result.extend_from_slice(contents);
    result
}

pub fn write_object(repo_root: &Path, hash: &str, data: &[u8]) -> Result<()> {
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

pub fn write_blob(repo_root: &Path, file_path: &Path) -> Result<String> {
    if !file_path.is_file() {
        bail!("Not a regular file: {}", file_path.display());
    }

    let content = fs::read(file_path)?;
    let object_bytes = format_object(&content);
    let hash = utils::hash_bytes(&object_bytes);

    write_object(repo_root, &hash, &object_bytes)?;
    Ok(hash)
}

pub fn hash_object(file_path: &Path) -> Result<String> {
    let repo_root = std::env::current_dir()?;
    write_blob(&repo_root, file_path)
}
