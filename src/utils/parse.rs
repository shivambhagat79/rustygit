use crate::commands::TreeEntry;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

pub fn parse_blob(root_path: &Path, blob_hash: &str) -> Result<String> {
    let blob_path = root_path
        .join(".rustygit/objects")
        .join(&blob_hash[..2])
        .join(&blob_hash[2..]);

    let blob_content = fs::read(&blob_path)?;
    let blob_str = String::from_utf8(blob_content)?;

    let nul_idx = blob_str
        .find('\0')
        .ok_or_else(|| anyhow!("Blob object missing NUL separator"))?;

    let blob_content = blob_str[nul_idx + 1..].to_string();

    Ok(blob_content)
}

pub fn parse_commit(root_path: &Path, commit_hash: &str) -> Result<String> {
    let commit_hash_path = root_path
        .join(".rustygit/objects")
        .join(&commit_hash[..2])
        .join(&commit_hash[2..]);

    let commit_content = fs::read(&commit_hash_path)?;
    let commit_str = String::from_utf8(commit_content)?;

    let nul_idx = commit_str
        .find('\0')
        .ok_or_else(|| anyhow!("Commit object missing NUL separator"))?;

    let (metadata, _) = commit_str.split_at(nul_idx);

    if !metadata.trim().starts_with("commit") {
        return Err(anyhow!(
            "Error parsing commit object.\nHash: {}",
            commit_hash
        ));
    }

    let commit_content = commit_str[nul_idx + 1..].to_string();

    Ok(commit_content)
}

pub fn parse_tree(root_path: &Path, tree_hash: &str) -> Result<Vec<TreeEntry>> {
    let tree_path = root_path
        .join(".rustygit/objects")
        .join(&tree_hash[..2])
        .join(&tree_hash[2..]);

    // NOTE: Tree objects are not valid UTF-8 because they embed 20 raw hash bytes per entry.
    // So parse as bytes, and only decode the textual portions (mode + name) as UTF-8.
    let tree_bytes = fs::read(&tree_path)?;

    let nul_idx = tree_bytes
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| anyhow!("Tree object missing NUL separator"))?;

    let (tree_metadata_bytes, mut rest) = tree_bytes.split_at(nul_idx);

    let tree_metadata = std::str::from_utf8(tree_metadata_bytes)
        .map_err(|e| anyhow!("Tree object header is not valid UTF-8: {e}"))?;

    if !tree_metadata.trim().starts_with("tree") {
        return Err(anyhow!("Error parsing tree object.\nHash: {}", tree_hash));
    }

    // Skip the NUL separator (rest currently starts with that NUL)
    rest = &rest[1..];

    let mut tree_entries: Vec<TreeEntry> = Vec::new();

    // Entry format produced by `format_tree` in `tree.rs`:
    // "{mode} {name}\0" + 20 bytes hash + '\n'
    while !rest.is_empty() {
        // Find the end of the "{mode} {name}" header (NUL)
        let entry_nul = rest
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| anyhow!("Malformed tree entry: missing NUL after mode/name"))?;

        let header_bytes = &rest[..entry_nul];
        let header = std::str::from_utf8(header_bytes)
            .map_err(|e| anyhow!("Malformed tree entry header (non-UTF8): {e}"))?;

        let (mode, name) = header
            .split_once(' ')
            .ok_or_else(|| anyhow!("Malformed tree entry header (expected \"<mode> <name>\")"))?;

        let mode: &'static str = match mode {
            "100644" => "100644",
            "40000" => "40000",
            other => return Err(anyhow!("Unsupported tree entry mode: {other}")),
        };

        // Move past "{mode} {name}\0"
        rest = &rest[entry_nul + 1..];

        // Next 20 bytes are the raw hash
        if rest.len() < 20 {
            return Err(anyhow!("Malformed tree entry: not enough bytes for hash"));
        }
        let hash_slice = &rest[..20];
        let mut hash = [0u8; 20];
        hash.copy_from_slice(hash_slice);
        rest = &rest[20..];

        tree_entries.push(TreeEntry {
            mode,
            name: name.to_string(),
            hash,
        });
    }

    Ok(tree_entries)
}
