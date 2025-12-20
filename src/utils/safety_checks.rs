use crate::utils;
use anyhow::{Result, bail};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn checkout_safety_check(root_path: &Path, target_tree_hash: Option<String>) -> Result<()> {
    let current_tree_hash = utils::get_current_tree_hash(root_path)?;

    let mut current_tree_map: HashMap<PathBuf, String> = HashMap::new();
    let mut target_tree_map: HashMap<PathBuf, String> = HashMap::new();
    let mut working_dir_map: HashMap<PathBuf, String> = HashMap::new();

    if let Some(hash) = current_tree_hash {
        utils::get_tree_files_map(root_path, Path::new(""), &hash, &mut current_tree_map)?;
    }

    if let Some(hash) = target_tree_hash {
        utils::get_tree_files_map(root_path, Path::new(""), &hash, &mut target_tree_map)?;
    }

    utils::get_work_dir_map(root_path, Path::new(""), &mut working_dir_map)?;

    for (path, work_hash) in working_dir_map.iter() {
        let in_current = current_tree_map.get(path);
        let in_target = target_tree_map.get(path);

        if in_current.is_none() && in_target.is_some() {
            bail!(
                "Untracked file '{}' would be overwritten by checkout",
                path.display()
            );
        }

        if let Some(cur_hash) = in_current {
            if work_hash != cur_hash {
                match in_target {
                    Some(target_hash) if target_hash == cur_hash => {}
                    _ => {
                        bail!(
                            "Local changes to '{}' would be overwritten by checkout",
                            path.display()
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
