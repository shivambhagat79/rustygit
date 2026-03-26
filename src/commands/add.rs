use crate::{commands, utils};
use anyhow::{Result, anyhow, bail};
use std::{collections::HashMap, path::Path};

fn add_all(root_path: &Path) -> Result<()> {
    let ignore_rules = utils::parse_ignore_file(root_path)?;

    let mut work_dir_map = HashMap::new();
    utils::get_work_dir_map(root_path, Path::new(""), &mut work_dir_map)?;

    let mut index_map = utils::read_index_map(root_path)?;

    for (path, _) in work_dir_map {
        let full_path = root_path.join(&path);

        if utils::is_ignored(&full_path, root_path, &ignore_rules) {
            continue;
        }

        let blob_hash = commands::write_blob(root_path, &full_path)?;
        index_map.insert(path, blob_hash);
    }

    utils::write_index_map(root_path, &index_map)?;
    Ok(())
}

pub fn add(root_path: &Path, file: &Path) -> Result<()> {
    utils::ensure_repo_exists(root_path)?;

    if file == Path::new(".") {
        return add_all(root_path);
    }

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
