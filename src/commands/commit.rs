use crate::utils::IgnoreRule;
use crate::{commands, utils};
use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

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

fn build_commit(
    path: &Path,
    message: String,
    ignore_rules: &Vec<IgnoreRule>,
) -> Result<CommitObject> {
    let tree_hash = commands::write_tree(path, path, ignore_rules)?;
    let mut parent: Option<String> = None;
    let author = User {
        name: String::from("Shivam Bhagat"),
        email: String::from("shivambhagat@rustygit.com"),
    };
    let committer = author.clone();

    let head_path = path.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;
    let ref_path = head_content[5..].trim();

    let head_ref_path = path.join(".rustygit").join(ref_path);

    if head_ref_path.exists() {
        let parent_content = fs::read_to_string(&head_ref_path)?;
        if !parent_content.trim().is_empty() {
            parent = Some(parent_content.trim().to_string());
        }
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
    let head_path = repo_root.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;
    let ref_path = head_content[5..].trim();

    let head_ref_path = repo_root.join(".rustygit").join(ref_path);

    fs::write(head_ref_path, format!("{}\n", commit_hash))?;
    Ok(())
}

pub fn commit(path: &Path, message: String, ignore_rules: &Vec<IgnoreRule>) -> Result<String> {
    utils::ensure_repo_exists(&path)?;

    // ensure head is attached
    let head_path = path.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;

    if !head_content.starts_with("ref: ") {
        bail!("Cannot commit: HEAD is detached.");
    }

    let commit_object = build_commit(path, message, ignore_rules)?;
    let data = format_commit(commit_object);
    let mut content: Vec<u8> = Vec::new();

    content.extend_from_slice(format!("commit {}\0", data.len()).as_bytes());
    content.extend_from_slice(&data);

    let hash = utils::hash_bytes(&content);
    commands::write_object(path, &hash, &content)?;

    update_head(path, &hash)?;

    Ok(hash)
}
