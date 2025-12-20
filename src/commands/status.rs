use crate::utils::{self, IgnoreRule};
use anyhow::{Result, bail};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub fn status(root_path: &Path, ignore_rules: &Vec<IgnoreRule>) -> Result<String> {
    let mut work_dir_map: HashMap<PathBuf, String> = HashMap::new();
    utils::get_work_dir_map(root_path, Path::new(""), &mut work_dir_map)?;

    let cur_tree_hash = utils::get_current_tree_hash(root_path)?;
    let mut cur_tree_map: HashMap<PathBuf, String> = HashMap::new();
    if let Some(hash) = cur_tree_hash {
        utils::get_tree_files_map(root_path, Path::new(""), &hash, &mut cur_tree_map)?;
    }

    let mut clean: bool = true;
    let mut modified_files: Vec<PathBuf> = Vec::new();
    let mut deleted_files: Vec<PathBuf> = Vec::new();
    let mut untracked_files: Vec<PathBuf> = Vec::new();

    for (path, work_hash) in work_dir_map.iter() {
        if utils::is_ignored(&root_path.join(path), root_path, ignore_rules) {
            continue;
        }
        let in_current = cur_tree_map.get(path);

        match in_current {
            Some(cur_hash) => {
                if work_hash != cur_hash {
                    modified_files.push(path.clone());
                    clean = false;
                }
            }
            None => {
                untracked_files.push(path.clone());
                clean = false;
            }
        }
    }

    for (path, _) in cur_tree_map.iter() {
        if utils::is_ignored(&root_path.join(path), root_path, ignore_rules) {
            continue;
        }
        let in_work_dir = work_dir_map.get(path);

        if in_work_dir.is_none() {
            deleted_files.push(path.clone());
            clean = false;
        }
    }

    let head_path = root_path.join(".rustygit").join("HEAD");
    let head_content = fs::read_to_string(&head_path)?;
    let mut output_string = String::new();

    if !head_content.starts_with("ref: ") {
        output_string.push_str(&format!(
            "Branch: None\tHEAD is in a DETACHED state on commit: {}\n\n",
            head_content
        ));
    } else {
        let ref_path = head_content[5..].trim();
        let branch_name = if ref_path.starts_with("refs/heads/") {
            &ref_path["refs/heads/".len()..]
        } else {
            bail!("HEAD could not be resolved.")
        };
        output_string.push_str(&format!("On Branch: {}\n\n", branch_name));
    }

    if clean {
        output_string.push_str("Working directory clean.\n");
    } else {
        println!("Changes not staged for commit:");
        if !modified_files.is_empty() {
            output_string.push_str("\tModified files:\n");
            for file in modified_files {
                output_string.push_str(&format!("\t\t{}\n", file.display()));
            }
        }

        if !deleted_files.is_empty() {
            output_string.push_str("\tDeleted files:\n");
            for file in deleted_files {
                output_string.push_str(&format!("\t\t{}\n", file.display()));
            }
        }

        if !untracked_files.is_empty() {
            output_string.push_str("\tUntracked files:\n");
            for file in untracked_files {
                output_string.push_str(&format!("\t\t{}\n", file.display()));
            }
        }
    }

    Ok(output_string)
}
