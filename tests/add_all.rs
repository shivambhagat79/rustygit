use rustygit::{commands, utils};
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

fn index_paths(repo_root: &Path) -> HashMap<std::path::PathBuf, String> {
    utils::read_index_map(repo_root).unwrap()
}

#[test]
fn add_dot_stages_multiple_files() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"a").unwrap();
    fs::write(repo_root.join("b.txt"), b"b").unwrap();

    commands::add(&repo_root, Path::new(".")).unwrap();

    let index_map = index_paths(&repo_root);
    assert!(index_map.contains_key(Path::new("a.txt")));
    assert!(index_map.contains_key(Path::new("b.txt")));
}

#[test]
fn add_dot_skips_ignored_files() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(
        repo_root.join(".rustygitignore"),
        b"ignored.txt\n.rustygitignore",
    )
    .unwrap();
    fs::write(repo_root.join("tracked.txt"), b"tracked").unwrap();
    fs::write(repo_root.join("ignored.txt"), b"ignored").unwrap();

    commands::add(&repo_root, Path::new(".")).unwrap();

    let index_map = index_paths(&repo_root);
    assert!(index_map.contains_key(Path::new("tracked.txt")));
    assert!(!index_map.contains_key(Path::new("ignored.txt")));
}

#[test]
fn add_dot_stages_nested_directories() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join("src").join("nested")).unwrap();
    fs::write(repo_root.join("src").join("main.rs"), b"fn main() {}\n").unwrap();
    fs::write(
        repo_root.join("src").join("nested").join("lib.rs"),
        b"pub fn x() {}\n",
    )
    .unwrap();

    commands::add(&repo_root, Path::new(".")).unwrap();

    let index_map = index_paths(&repo_root);
    assert!(index_map.contains_key(Path::new("src/main.rs")));
    assert!(index_map.contains_key(Path::new("src/nested/lib.rs")));
}
