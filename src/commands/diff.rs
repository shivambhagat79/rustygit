//! Working-directory diff rendering against HEAD-tracked content.

use crate::utils::{self, IgnoreRule};
use anyhow::Result;
use similar::TextDiff;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Prints line-level differences between working directory files and HEAD.
pub fn diff(root_path: &Path, ignore_rules: &Vec<IgnoreRule>) -> Result<()> {
    let mut work_dir_map: HashMap<PathBuf, String> = HashMap::new();
    let cur_tree_hash = utils::get_current_tree_hash(root_path)?;

    utils::get_work_dir_map(root_path, Path::new(""), &mut work_dir_map)?;

    let mut cur_tree_map: HashMap<PathBuf, String> = HashMap::new();
    if let Some(hash) = cur_tree_hash {
        utils::get_tree_files_map(root_path, Path::new(""), &hash, &mut cur_tree_map)?;
    }

    let mut modified_files: Vec<PathBuf> = Vec::new();
    let mut untracked_files: Vec<PathBuf> = Vec::new();
    let mut deleted_files: Vec<PathBuf> = Vec::new();

    for (path, work_hash) in work_dir_map.iter() {
        if utils::is_ignored(&root_path.join(path), root_path, ignore_rules) {
            continue;
        }
        let in_current = cur_tree_map.get(path);

        match in_current {
            Some(cur_hash) => {
                if work_hash != cur_hash {
                    modified_files.push(path.clone());
                }
            }
            None => {
                untracked_files.push(path.clone());
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
        }
    }

    if untracked_files.is_empty() && modified_files.is_empty() && deleted_files.is_empty() {
        println!("No changes.");
        return Ok(());
    }

    if !untracked_files.is_empty() {
        println!("\nNew files Created:");
        for file in untracked_files {
            let lines = std::fs::read_to_string(root_path.join(&file))?
                .lines()
                .count();
            println!("\t{} (+{} lines)", file.display(), lines);
        }
    }

    if !modified_files.is_empty() {
        println!("\nModified files:");
        for file in modified_files {
            println!("\t{}", file.display());
            let work_file_string = std::fs::read_to_string(root_path.join(&file))?;
            let tree_file_string = utils::parse_blob(root_path, &cur_tree_map[&file])?;

            let diff = TextDiff::from_lines(&tree_file_string, &work_file_string);

            for change in diff.iter_all_changes() {
                match change.tag() {
                    similar::ChangeTag::Delete => {
                        if let Some(line_no) = change.old_index() {
                            print!("\t\t-{:>4} | {}", line_no + 1, change);
                        }
                    }
                    similar::ChangeTag::Insert => {
                        if let Some(line_no) = change.new_index() {
                            print!("\t\t+{:>4} | {}", line_no + 1, change);
                        }
                    }
                    similar::ChangeTag::Equal => {}
                }
            }
        }
    }

    if !deleted_files.is_empty() {
        println!("\nDeleted files:");
        for file in deleted_files {
            let lines = utils::parse_blob(root_path, &cur_tree_map[&file])?
                .lines()
                .count();
            println!("\t{} (-{} lines)", file.display(), lines);
        }
    }
    Ok(())
}
