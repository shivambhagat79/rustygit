use rustygit::{commands, utils};
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

#[test]
fn remove_file_then_commit_removes_from_tree() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();

    commands::rm(&repo_root, &repo_root.join("a.txt")).unwrap();
    commands::commit(&repo_root, "remove a".to_string(), &vec![]).unwrap();

    let tree_hash = utils::get_current_tree_hash(&repo_root)
        .unwrap()
        .expect("expected head tree");
    let mut tree_map = HashMap::new();
    utils::get_tree_files_map(&repo_root, Path::new(""), &tree_hash, &mut tree_map).unwrap();

    assert!(!tree_map.contains_key(Path::new("a.txt")));
}

#[test]
fn remove_fails_on_modified_file() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    let result = commands::rm(&repo_root, &repo_root.join("a.txt"));

    assert!(result.is_err());
}

#[test]
fn remove_updates_index() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();

    commands::rm(&repo_root, &repo_root.join("a.txt")).unwrap();

    let index_map = utils::read_index_map(&repo_root).unwrap();
    assert!(!index_map.contains_key(Path::new("a.txt")));
}

#[test]
fn remove_non_existent_file_fails() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();

    let result = commands::rm(&repo_root, Path::new("missing.txt"));
    assert!(result.is_err());
}
