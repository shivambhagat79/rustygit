use anyhow::{Result, bail};
use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Blob,
}

impl ObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Blob => "blob",
        }
    }
}

fn format_object(object_type: ObjectType, contents: &[u8]) -> Vec<u8> {
    let header = format!("{} {}\0", object_type.as_str(), contents.len());
    let mut formatted = Vec::new();

    formatted.extend_from_slice(header.as_bytes());
    formatted.extend_from_slice(contents);

    formatted
}

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn write_object(path: &Path, hash: &str, data: &[u8]) -> Result<()> {
    let objet_dir = path.join(".rustygit").join("objects");

    let (dir_name, file_name) = hash.split_at(2);

    let object_dir = objet_dir.join(dir_name);
    let object_path = object_dir.join(file_name);

    if object_path.exists() {
        return Ok(());
    }

    fs::create_dir_all(object_dir)?;
    fs::write(object_path, data)?;

    Ok(())
}

pub fn hash_object(file_path: &Path) -> Result<String> {
    if !file_path.exists() {
        bail!("File does not exist: {}", file_path.display());
    }

    let file_content = fs::read(file_path)?;

    let object_content = format_object(ObjectType::Blob, &file_content);

    let hash = hash_bytes(&object_content);
    println!("{}", &hash);

    let repo_path = std::env::current_dir()?;
    write_object(&repo_path, &hash, &object_content)?;

    Ok(hash)
}
