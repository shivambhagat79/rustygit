use rustygit::commands;
use std::fs;
use tempfile::tempdir;

#[test]
fn restore_modified_file_resets_content() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::restore(&repo_root, &repo_root.join("a.txt")).unwrap();

    let content = fs::read_to_string(repo_root.join("a.txt")).unwrap();
    assert_eq!(content, "one");
}

#[test]
fn restore_deleted_file_brings_it_back() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();

    fs::remove_file(repo_root.join("a.txt")).unwrap();
    commands::restore(&repo_root, &repo_root.join("a.txt")).unwrap();

    let content = fs::read_to_string(repo_root.join("a.txt")).unwrap();
    assert_eq!(content, "one");
}

#[test]
fn restore_untracked_file_deletes_it() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();
    fs::write(repo_root.join("untracked.txt"), b"temp").unwrap();

    commands::restore(&repo_root, &repo_root.join("untracked.txt")).unwrap();

    assert!(!repo_root.join("untracked.txt").exists());
}
