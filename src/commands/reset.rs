//! HEAD/index reset operations (`--soft` and mixed/default).

use crate::utils;
use anyhow::{Result, bail};
use std::{fs, path::Path};

fn resolve_commit(root_path: &Path, target: &str) -> Result<String> {
    if target.len() != 40 {
        bail!("Invalid commit hash '{}'.", target);
    }

    let object_path = root_path
        .join(".rustygit")
        .join("objects")
        .join(&target[..2])
        .join(&target[2..]);

    if !object_path.exists() {
        bail!("Commit '{}' does not exist.", target);
    }

    let _ = utils::parse_commit(root_path, target)?;
    Ok(target.to_string())
}

fn update_head_to_commit(root_path: &Path, commit_hash: &str) -> Result<()> {
    let head_path = root_path.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;

    if head_content.starts_with("ref: ") {
        let ref_path = head_content[5..].trim();
        let branch_ref_path = root_path.join(".rustygit").join(ref_path);
        fs::write(branch_ref_path, format!("{}\n", commit_hash))?;
    } else {
        fs::write(head_path, format!("{}\n", commit_hash))?;
    }

    Ok(())
}

fn set_index_to_commit_tree(root_path: &Path, commit_hash: &str) -> Result<()> {
    let commit_content = utils::parse_commit(root_path, commit_hash)?;
    let n_idx = commit_content
        .find('\n')
        .ok_or_else(|| anyhow::anyhow!("Malformed commit object: missing newline after tree"))?;

    let tree_hash = &commit_content[5..n_idx];

    let mut tree_map = std::collections::HashMap::new();
    utils::get_tree_files_map(root_path, Path::new(""), tree_hash, &mut tree_map)?;
    utils::write_index_map(root_path, &tree_map)?;

    Ok(())
}

/// Resets repository references to `target`.
///
/// - `soft = true`: move HEAD only (index/worktree unchanged)
/// - `soft = false`: move HEAD and reset index to target commit tree
pub fn reset(root_path: &Path, target: &str, soft: bool) -> Result<()> {
    utils::ensure_repo_exists(root_path)?;

    let target_commit = resolve_commit(root_path, target)?;

    if soft {
        let index_map = utils::read_index_map(root_path)?;
        if index_map.is_empty() {
            if let Some(current_commit) = utils::get_current_commit_hash(root_path)? {
                set_index_to_commit_tree(root_path, &current_commit)?;
            }
        }
    }

    update_head_to_commit(root_path, &target_commit)?;

    if !soft {
        set_index_to_commit_tree(root_path, &target_commit)?;
    }

    Ok(())
}
