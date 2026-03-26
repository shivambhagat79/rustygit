//! Commit creation and index-to-history transitions.

use crate::utils::IgnoreRule;
use crate::{commands, utils};
use anyhow::{Result, bail};
use std::fs;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

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
    let empty_ignore_rules = Vec::new();
    let tree_hash = commands::write_tree(path, path, &empty_ignore_rules)?;
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

fn get_current_tree_map(root_path: &Path) -> Result<HashMap<PathBuf, String>> {
    let mut current_tree_map = HashMap::new();

    if let Some(tree_hash) = utils::get_current_tree_hash(root_path)? {
        utils::get_tree_files_map(root_path, Path::new(""), &tree_hash, &mut current_tree_map)?;
    }

    Ok(current_tree_map)
}

fn ensure_index_has_head_snapshot(root_path: &Path) -> Result<()> {
    let index_map = utils::read_index_map(root_path)?;
    if !index_map.is_empty() {
        return Ok(());
    }

    let current_tree_map = get_current_tree_map(root_path)?;
    if current_tree_map.is_empty() {
        return Ok(());
    }

    utils::write_index_map(root_path, &current_tree_map)?;
    Ok(())
}

fn auto_stage_tracked_files(root_path: &Path, ignore_rules: &Vec<IgnoreRule>) -> Result<()> {
    // `commit -a` stages tracked modifications and deletions by comparing
    // the current HEAD tree with working-directory hashes.
    // Untracked paths are intentionally ignored.
    let current_tree_map = get_current_tree_map(root_path)?;
    let mut work_dir_map = HashMap::new();
    utils::get_work_dir_map(root_path, Path::new(""), &mut work_dir_map)?;

    let mut index_map = utils::read_index_map(root_path)?;

    for (path, hash) in &current_tree_map {
        index_map.entry(path.clone()).or_insert(hash.clone());
    }

    for (path, current_hash) in &current_tree_map {
        if utils::is_ignored(&root_path.join(path), root_path, ignore_rules) {
            continue;
        }

        match work_dir_map.get(path) {
            Some(work_hash) => {
                if work_hash != current_hash {
                    let blob_hash = commands::write_blob(root_path, &root_path.join(path))?;
                    index_map.insert(path.clone(), blob_hash);
                }
            }
            None => {
                index_map.remove(path);
            }
        }
    }

    utils::write_index_map(root_path, &index_map)?;
    Ok(())
}

/// Creates a commit from the index, optionally auto-staging tracked changes.
///
/// When `all` is true, tracked modifications and deletions are staged first,
/// matching the behavior of `commit -a`.
pub fn commit_with_all(
    path: &Path,
    message: String,
    ignore_rules: &Vec<IgnoreRule>,
    all: bool,
) -> Result<String> {
    utils::ensure_repo_exists(&path)?;

    // ensure head is attached
    let head_path = path.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;

    if !head_content.starts_with("ref: ") {
        bail!("Cannot commit: HEAD is detached.");
    }

    if all {
        auto_stage_tracked_files(path, ignore_rules)?;
    } else {
        ensure_index_has_head_snapshot(path)?;
    }

    let commit_object = build_commit(path, message)?;
    let data = format_commit(commit_object);
    let mut content: Vec<u8> = Vec::new();

    content.extend_from_slice(format!("commit {}\0", data.len()).as_bytes());
    content.extend_from_slice(&data);

    let hash = utils::hash_bytes(&content);
    commands::write_object(path, &hash, &content)?;

    update_head(path, &hash)?;
    utils::clear_index(path)?;

    Ok(hash)
}

/// Creates a commit from the index.
///
/// This variant does not auto-stage working directory changes.
pub fn commit(path: &Path, message: String, ignore_rules: &Vec<IgnoreRule>) -> Result<String> {
    commit_with_all(path, message, ignore_rules, false)
}
