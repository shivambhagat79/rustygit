use rustygit::commands;
use std::fs;
use tempfile::tempdir;

#[test]
fn restore_file() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // create a file
    fs::write(repo_root.join("a.txt"), b"one").unwrap();

    let hash_first = commands::commit(&repo_root, String::from("First"), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"Hello Again").unwrap();

    commands::commit(&repo_root, String::from("two"), &vec![]).unwrap();

    commands::checkout(&repo_root, &hash_first).unwrap();

    let content = fs::read_to_string(repo_root.join("a.txt")).unwrap();

    assert_eq!(content, "one");
}

#[test]
fn restore_directory() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // create a directory with a file
    fs::create_dir_all(repo_root.join("dir")).unwrap();
    fs::write(repo_root.join("dir").join("a.txt"), b"one").unwrap();

    let hash_first = commands::commit(&repo_root, String::from("First"), &vec![]).unwrap();

    fs::write(repo_root.join("dir").join("a.txt"), b"two").unwrap();

    commands::commit(&repo_root, String::from("Second"), &vec![]).unwrap();

    commands::checkout(&repo_root, &hash_first).unwrap();

    let content = fs::read_to_string(repo_root.join("dir").join("a.txt")).unwrap();

    assert_eq!(content, "one");
}

#[test]
fn head_is_detached() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // create a file
    fs::write(repo_root.join("a.txt"), b"one").unwrap();

    let hash_first = commands::commit(&repo_root, String::from("First"), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"Hello Again").unwrap();

    commands::commit(&repo_root, String::from("two"), &vec![]).unwrap();

    commands::checkout(&repo_root, &hash_first).unwrap();

    let content = fs::read_to_string(repo_root.join(".rustygit/HEAD")).unwrap();

    assert_eq!(content, hash_first);
}
