use anyhow::Result;
use glob::Pattern;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum IgnoreRule {
    File(PathBuf),
    Directory(PathBuf),
    Glob(Pattern),
}

pub fn parse_ignore_file(root_path: &Path) -> Result<Vec<IgnoreRule>> {
    let ignore_file_path = root_path.join(".rustygitignore");
    let mut ignore_rules: Vec<IgnoreRule> = Vec::new();

    if !ignore_file_path.exists() {
        println!("no file");
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(ignore_file_path)?;

    for raw in content.lines() {
        let line = raw.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.contains(' ') || line.contains("**") || line.contains('!') {
            continue;
        }

        if line.ends_with('/') {
            ignore_rules.push(IgnoreRule::Directory(PathBuf::from(
                line.trim_end_matches('/'),
            )));
            continue;
        }

        if line.contains('*') {
            if let Ok(pattern) = Pattern::new(line) {
                ignore_rules.push(IgnoreRule::Glob(pattern));
            }
            continue;
        }

        ignore_rules.push(IgnoreRule::File(PathBuf::from(line)));
    }

    for rule in &ignore_rules {
        println!("Ignore Rule: {:?}", rule);
    }

    Ok(ignore_rules)
}

pub fn is_ignored(path: &Path, root_path: &Path, ignore_rules: &Vec<IgnoreRule>) -> bool {
    let path = path.strip_prefix(root_path).unwrap_or(path);
    for rule in ignore_rules {
        match rule {
            IgnoreRule::File(file_path) => {
                if path == file_path || path.ends_with(file_path) {
                    return true;
                }
            }
            IgnoreRule::Directory(dir_path) => {
                if path == dir_path || path.starts_with(dir_path) {
                    return true;
                }
            }
            IgnoreRule::Glob(pattern) => {
                if let Some(path_str) = path.to_str() {
                    let path_str = path_str.replace('\\', "/");
                    if pattern.matches(&path_str) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
