use crate::utils;
use anyhow::{Result, anyhow, bail};
use std::{fs, path::Path};

pub fn restore(root_path: &Path, file: &Path) -> Result<()> {
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

    let index_map = utils::read_index_map(root_path)?;

    if let Some(blob_hash) = index_map.get(&relative_path) {
        let content = utils::parse_blob(root_path, blob_hash)?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(file_path, content)?;
        return Ok(());
    }

    if file_path.exists() {
        if !file_path.is_file() {
            bail!("Could not restore '{}': not a file.", file.display());
        }
        fs::remove_file(file_path)?;
    }

    Ok(())
}
