use std::fs;
use tempfile::tempdir;

use rustygit::{object, repo, utils};

#[test]
fn initial_commit_creates_commit_object() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    repo::init(&repo_root).unwrap();

    // create a file
    fs::write(repo_root.join("a.txt"), b"hello").unwrap();

    // commit
    utils::ensure_repo_exists(&repo_root).unwrap();
    let commit_hash = object::commit(&repo_root, "initial commit".to_string()).unwrap();

    // commit object exists
    let (d, f) = commit_hash.split_at(2);
    let commit_object = repo_root.join(".rustygit").join("objects").join(d).join(f);

    assert!(commit_object.exists());
}

#[test]
fn second_commit_has_parent() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    repo::init(&repo_root).unwrap();

    fs::write(repo_root.join("file.txt"), b"one").unwrap();
    let first_commit = object::commit(&repo_root, "first".to_string()).unwrap();

    fs::write(repo_root.join("file.txt"), b"two").unwrap();
    let second_commit = object::commit(&repo_root, "second".to_string()).unwrap();

    let (d, f) = second_commit.split_at(2);
    let commit_path = repo_root.join(".rustygit").join("objects").join(d).join(f);

    let contents = fs::read(commit_path).unwrap();
    let contents_str = String::from_utf8_lossy(&contents);

    assert!(contents_str.contains(&first_commit));
}

#[test]
fn commit_fails_without_repo() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    fs::write(repo_root.join("file.txt"), b"hello").unwrap();

    let result = object::commit(&repo_root, "should fail".to_string());

    assert!(result.is_err());
}
