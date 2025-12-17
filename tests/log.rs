use rustygit::commands;
use std::fs;
use tempfile::tempdir;

#[test]
fn log_single_commit() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    commands::init(&root).unwrap();
    fs::write(root.join("file.txt"), "hello").unwrap();

    commands::commit(&root, "first".to_string(), &vec![]).unwrap();

    let result = commands::log(&root);
    assert!(result.is_ok());
}

#[test]
fn log_fails_without_repo() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();

    let result = commands::log(&root);
    assert!(result.is_err());
}

#[test]
fn log_fails_on_missing_parent_object() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    commands::init(&root).unwrap();
    fs::write(root.join("file.txt"), "hello").unwrap();

    let commit = commands::commit(&root, "first".to_string(), &vec![]).unwrap();

    // delete commit object
    let path = root
        .join(".rustygit/objects")
        .join(&commit[..2])
        .join(&commit[2..]);

    fs::remove_file(path).unwrap();

    let result = commands::log(&root);
    assert!(result.is_err());
}
