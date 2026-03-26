use rustygit::{commands, utils};
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

fn head_tree_map(repo_root: &Path) -> HashMap<std::path::PathBuf, String> {
    let tree_hash = utils::get_current_tree_hash(repo_root)
        .unwrap()
        .expect("expected head tree");

    let mut map = HashMap::new();
    utils::get_tree_files_map(repo_root, Path::new(""), &tree_hash, &mut map).unwrap();
    map
}

#[test]
fn commit_all_stages_modified_file() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    commands::commit(&repo_root, "first".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::commit_with_all(&repo_root, "second".to_string(), &vec![], true).unwrap();

    let tree_map = head_tree_map(&repo_root);
    let blob_hash = tree_map.get(Path::new("a.txt")).unwrap();
    let content = utils::parse_blob(&repo_root, blob_hash).unwrap();

    assert_eq!(content, "two");
}

#[test]
fn commit_all_ignores_untracked_files() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("tracked.txt"), b"tracked").unwrap();
    commands::add(&repo_root, &repo_root.join("tracked.txt")).unwrap();
    commands::commit(&repo_root, "first".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("new.txt"), b"new").unwrap();
    commands::commit_with_all(&repo_root, "second".to_string(), &vec![], true).unwrap();

    let tree_map = head_tree_map(&repo_root);
    assert!(tree_map.contains_key(Path::new("tracked.txt")));
    assert!(!tree_map.contains_key(Path::new("new.txt")));
}

#[test]
fn commit_all_handles_deletion() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"tracked").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    commands::commit(&repo_root, "first".to_string(), &vec![]).unwrap();

    fs::remove_file(repo_root.join("a.txt")).unwrap();
    commands::commit_with_all(&repo_root, "second".to_string(), &vec![], true).unwrap();

    let tree_map = head_tree_map(&repo_root);
    assert!(!tree_map.contains_key(Path::new("a.txt")));
}

#[test]
fn commit_without_all_does_not_auto_stage() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    commands::commit(&repo_root, "first".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::commit(&repo_root, "second".to_string(), &vec![]).unwrap();

    let tree_map = head_tree_map(&repo_root);
    let blob_hash = tree_map.get(Path::new("a.txt")).unwrap();
    let content = utils::parse_blob(&repo_root, blob_hash).unwrap();

    assert_eq!(content, "one");
}
