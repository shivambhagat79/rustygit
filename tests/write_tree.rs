use std::fs;
use tempfile::tempdir;

use rustygit::{commands, commands::write_tree, utils};

#[test]
fn write_tree_single_file() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    // init repository
    commands::init(&repo_root).unwrap();

    // create file
    fs::write(repo_root.join("a.txt"), b"hello").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();

    let ignore_rules = utils::parse_ignore_file(&repo_root).unwrap();
    let tree_hash = write_tree(&repo_root, &repo_root, &ignore_rules).unwrap();

    let (d, f) = tree_hash.split_at(2);
    let tree_object = repo_root.join(".rustygit").join("objects").join(d).join(f);

    assert!(tree_object.exists());
}

#[test]
fn write_tree_nested_directory() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join("src")).unwrap();
    fs::write(repo_root.join("src").join("main.rs"), b"fn main() {}").unwrap();
    commands::add(&repo_root, &repo_root.join("src").join("main.rs")).unwrap();

    let ignore_rules = utils::parse_ignore_file(&repo_root).unwrap();
    let tree_hash = write_tree(&repo_root, &repo_root, &ignore_rules).unwrap();

    let (d, f) = tree_hash.split_at(2);
    let tree_object = repo_root.join(".rustygit").join("objects").join(d).join(f);

    assert!(tree_object.exists());
}
