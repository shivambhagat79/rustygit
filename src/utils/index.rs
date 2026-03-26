use anyhow::{Result, anyhow, bail};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn index_path(root_path: &Path) -> PathBuf {
    root_path.join(".rustygit").join("index")
}

pub fn read_index_map(root_path: &Path) -> Result<HashMap<PathBuf, String>> {
    let path = index_path(root_path);

    if !path.exists() {
        return Ok(HashMap::new());
    }

    let mut map: HashMap<PathBuf, String> = HashMap::new();
    let content = fs::read_to_string(path)?;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let (hash, rel_path) = line
            .split_once(' ')
            .ok_or_else(|| anyhow!("Malformed index entry: {}", line))?;

        if hash.len() != 40 {
            bail!("Malformed index entry hash: {}", line);
        }

        map.insert(PathBuf::from(rel_path), hash.to_string());
    }

    Ok(map)
}

pub fn write_index_map(root_path: &Path, map: &HashMap<PathBuf, String>) -> Result<()> {
    let mut entries: Vec<(&PathBuf, &String)> = map.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let mut content = String::new();
    for (path, hash) in entries {
        content.push_str(&format!("{} {}\n", hash, path.display()));
    }

    fs::write(index_path(root_path), content)?;
    Ok(())
}

pub fn stage_index_entry(root_path: &Path, path: &Path, hash: &str) -> Result<()> {
    let mut map = read_index_map(root_path)?;
    map.insert(path.to_path_buf(), hash.to_string());
    write_index_map(root_path, &map)
}

pub fn clear_index(root_path: &Path) -> Result<()> {
    fs::write(index_path(root_path), "")?;
    Ok(())
}
