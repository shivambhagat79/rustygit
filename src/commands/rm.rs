//! File removal command for staging deletions safely.

use crate::utils;
use anyhow::{Result, anyhow, bail};
use std::{collections::HashMap, fs, path::Path};

/// Removes `file` from index and working directory.
///
/// Refuses removal when the working copy differs from the staged index hash,
/// preventing accidental data loss.
pub fn rm(root_path: &Path, file: &Path) -> Result<()> {
    utils::ensure_repo_exists(root_path)?;

    let file_path = if file.is_absolute() {
        file.to_path_buf()
    } else {
        root_path.join(file)
    };

    let relative_path = file_path
        .strip_prefix(root_path)
        .map_err(|_| anyhow!("File must be inside the repository root."))?
        .to_path_buf();

    let mut index_map = utils::read_index_map(root_path)?;
    let Some(index_hash) = index_map.get(&relative_path).cloned() else {
        bail!(
            "Cannot remove '{}': file is not staged in index.",
            file.display()
        );
    };

    let mut work_dir_map = HashMap::new();
    utils::get_work_dir_map(root_path, Path::new(""), &mut work_dir_map)?;

    if let Some(work_hash) = work_dir_map.get(&relative_path) {
        if work_hash != &index_hash {
            bail!(
                "Cannot remove '{}': file has local modifications not staged in index.",
                file.display()
            );
        }
    }

    index_map.remove(&relative_path);
    utils::write_index_map(root_path, &index_map)?;

    if file_path.exists() {
        if !file_path.is_file() {
            bail!("Could not remove '{}': not a file.", file.display());
        }
        fs::remove_file(file_path)?;
    }

    Ok(())
}
