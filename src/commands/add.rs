use crate::{commands, utils};
use anyhow::{Result, anyhow, bail};
use std::path::Path;

pub fn add(root_path: &Path, file: &Path) -> Result<()> {
    utils::ensure_repo_exists(root_path)?;

    let file_path = if file.is_absolute() {
        file.to_path_buf()
    } else {
        root_path.join(file)
    };

    if !file_path.exists() || !file_path.is_file() {
        bail!("Could not find file: {}", file.display());
    }

    let relative_path = file_path
        .strip_prefix(root_path)
        .map_err(|_| anyhow!("File must be inside the repository root."))?;

    let blob_hash = commands::write_blob(root_path, &file_path)?;
    utils::stage_index_entry(root_path, relative_path, &blob_hash)?;

    Ok(())
}
