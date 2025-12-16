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
    Commit,
}

impl ObjectType {
    fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
            ObjectType::Commit => "commit",
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

// =============================================================================
// Commit handling
// =============================================================================

#[derive(Clone)]
struct User {
    name: String,
    email: String,
}

struct CommitObject {
    tree_hash: String,
    parent: Option<String>,
    author: User,
    committer: User,
    message: String,
    timestamp: i64,
    timezone: String,
}

fn build_commit(path: &Path, message: String) -> Result<CommitObject> {
    let tree_hash = write_tree(path)?;
    let mut parent: Option<String> = None;
    let author = User {
        name: String::from("Shivam Bhagat"),
        email: String::from("shivambhagat@rustygit.com"),
    };
    let committer = author.clone();

    let head_path = path
        .join(".rustygit")
        .join("refs")
        .join("heads")
        .join("main");

    if head_path.exists() {
        let parent_content = fs::read_to_string(&head_path)?;
        parent = Some(parent_content.trim().to_string());
    }

    let (timestamp, timezone) = utils::get_time();

    let commit_object = CommitObject {
        tree_hash,
        parent,
        author,
        committer,
        message,
        timestamp,
        timezone,
    };

    Ok(commit_object)
}

fn format_commit(commit_object: CommitObject) -> Vec<u8> {
    let mut formatted: Vec<u8> = Vec::new();

    formatted.extend_from_slice(format!("tree {}\n", commit_object.tree_hash).as_bytes());

    if let Some(parent) = &commit_object.parent {
        formatted.extend_from_slice(format!("parent {}\n", parent).as_bytes());
    }

    formatted.extend_from_slice(
        format!(
            "author {} <{}> {} {}\n",
            commit_object.author.name,
            commit_object.author.email,
            commit_object.timestamp,
            commit_object.timezone
        )
        .as_bytes(),
    );

    formatted.extend_from_slice(
        format!(
            "committer {} <{}> {} {}\n\n",
            commit_object.committer.name,
            commit_object.committer.email,
            commit_object.timestamp,
            commit_object.timezone
        )
        .as_bytes(),
    );

    formatted.extend_from_slice(commit_object.message.as_bytes());

    formatted
}

fn update_head(repo_root: &Path, commit_hash: &str) -> Result<()> {
    let head_ref_path = repo_root
        .join(".rustygit")
        .join("refs")
        .join("heads")
        .join("main");

    fs::write(head_ref_path, format!("{}\n", commit_hash))?;
    Ok(())
}

pub fn commit(path: &Path, message: String) -> Result<String> {
    utils::ensure_repo_exists(&path)?;
    let commit_object = build_commit(path, message)?;
    let data = format_commit(commit_object);
    let mut content: Vec<u8> = Vec::new();

    content.extend_from_slice(format!("commit {}\0", data.len()).as_bytes());
    content.extend_from_slice(&data);

    let hash = hash_bytes(&content);
    write_object(path, &hash, &content)?;

    update_head(path, &hash)?;

    Ok(hash)
}
