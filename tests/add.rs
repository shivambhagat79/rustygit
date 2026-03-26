use rustygit::{commands, utils};
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

fn head_file_content(repo_root: &Path, file: &str) -> String {
    let tree_hash = utils::get_current_tree_hash(repo_root)
        .unwrap()
        .expect("expected commit tree");

    let mut tree_map = HashMap::new();
    utils::get_tree_files_map(repo_root, Path::new(""), &tree_hash, &mut tree_map).unwrap();

    let blob_hash = tree_map
        .get(Path::new(file))
        .expect("expected file in committed tree");

    utils::parse_blob(repo_root, blob_hash).unwrap()
}

#[test]
fn add_file_writes_index_entry() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("file.txt"), b"hello").unwrap();

    commands::add(&repo_root, &repo_root.join("file.txt")).unwrap();

    let index_content = fs::read_to_string(repo_root.join(".rustygit").join("index")).unwrap();
    assert!(index_content.contains(" file.txt"));
}

#[test]
fn add_then_commit_uses_staged_version() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("file.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("file.txt")).unwrap();

    fs::write(repo_root.join("file.txt"), b"two").unwrap();
    commands::add(&repo_root, &repo_root.join("file.txt")).unwrap();

    commands::commit(&repo_root, "commit staged".to_string(), &vec![]).unwrap();

    let committed_content = head_file_content(&repo_root, "file.txt");
    assert_eq!(committed_content, "two");
}

#[test]
fn modify_after_add_commit_uses_original_staged_version() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("file.txt"), b"staged").unwrap();
    commands::add(&repo_root, &repo_root.join("file.txt")).unwrap();

    fs::write(repo_root.join("file.txt"), b"unstaged").unwrap();

    commands::commit(&repo_root, "commit staged snapshot".to_string(), &vec![]).unwrap();

    let committed_content = head_file_content(&repo_root, "file.txt");
    assert_eq!(committed_content, "staged");
}

#[test]
fn partial_staging_commits_only_staged_file() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"a").unwrap();
    fs::write(repo_root.join("b.txt"), b"b").unwrap();

    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    commands::commit(&repo_root, "partial".to_string(), &vec![]).unwrap();

    let tree_hash = utils::get_current_tree_hash(&repo_root)
        .unwrap()
        .expect("expected commit tree");

    let mut tree_map = HashMap::new();
    utils::get_tree_files_map(&repo_root, Path::new(""), &tree_hash, &mut tree_map).unwrap();

    assert!(tree_map.contains_key(Path::new("a.txt")));
    assert!(!tree_map.contains_key(Path::new("b.txt")));
}
