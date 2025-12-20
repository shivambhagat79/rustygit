use rustygit::{commands, utils};
use std::fs;
use tempfile::tempdir;

#[test]
fn clean_repo_status() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();

    let ignore_rules = vec![];
    let status = commands::status(&repo_root, &ignore_rules).unwrap();
    assert!(status.contains("Working directory clean."));

    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::commit(&repo_root, String::from("First"), &ignore_rules).unwrap();
    let status = commands::status(&repo_root, &ignore_rules).unwrap();
    assert!(status.contains("Working directory clean."));

    fs::write(repo_root.join("a.txt"), b"modified").unwrap();
    fs::write(repo_root.join(".rustygitignore"), b"a.txt\n.rustygitignore").unwrap();
    let ignore_rules = utils::parse_ignore_file(&repo_root).unwrap();
    let status = commands::status(&repo_root, &ignore_rules).unwrap();
    assert!(status.contains("Working directory clean."));
}

#[test]
fn modified_file_status() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();

    let ignore_rules = vec![];
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::commit(&repo_root, String::from("First"), &ignore_rules).unwrap();
    fs::write(repo_root.join("a.txt"), b"modified").unwrap();

    let status = commands::status(&repo_root, &ignore_rules).unwrap();

    assert!(status.contains("Modified files:\n\t\ta.txt"));
}

#[test]
fn unntracked_file_status() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();

    let ignore_rules = vec![];
    fs::write(repo_root.join("a.txt"), b"one").unwrap();

    let status = commands::status(&repo_root, &ignore_rules).unwrap();

    assert!(status.contains("Untracked files:\n\t\ta.txt"));
}

#[test]
fn ignored_file_status() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();

    fs::write(repo_root.join(".rustygitignore"), b"b.txt").unwrap();
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    fs::write(repo_root.join("b.txt"), b"Ignored").unwrap();

    let ignore_rules = utils::parse_ignore_file(&repo_root).unwrap();

    commands::commit(&repo_root, String::from("First"), &ignore_rules).unwrap();

    fs::write(repo_root.join("b.txt"), b"one").unwrap();

    let status = commands::status(&repo_root, &ignore_rules).unwrap();

    assert!(!status.contains("b.txt"));
    assert!(status.contains("Working directory clean."));
}

#[test]
fn deleted_file_status() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();

    fs::write(repo_root.join("a.txt"), b"one").unwrap();

    commands::commit(&repo_root, String::from("First"), &vec![]).unwrap();

    fs::remove_file(repo_root.join("a.txt")).unwrap();

    let status = commands::status(&repo_root, &vec![]).unwrap();

    assert!(status.contains("Deleted files:\n\t\ta.txt"));
}
