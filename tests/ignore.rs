use std::fs;
use tempfile::tempdir;

use rustygit::utils::{is_ignored, parse_ignore_file};

#[test]
fn ignore_specific_file() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    fs::write(root.join(".rustygitignore"), "secret.txt\n").unwrap();
    fs::write(root.join("secret.txt"), "nope").unwrap();
    fs::write(root.join("visible.txt"), "ok").unwrap();

    let rules = parse_ignore_file(&root).unwrap();

    assert!(is_ignored(&root.join("secret.txt"), &root, &rules));
    assert!(!is_ignored(&root.join("visible.txt"), &root, &rules));
}

#[test]
fn ignore_directory() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    fs::write(root.join(".rustygitignore"), "target/\n").unwrap();
    fs::create_dir_all(root.join("target")).unwrap();
    fs::create_dir_all(root.join("src")).unwrap();

    fs::write(root.join("target/file.o"), "obj").unwrap();
    fs::write(root.join("src/main.rs"), "code").unwrap();

    let rules = parse_ignore_file(&root).unwrap();

    assert!(is_ignored(&root.join("target"), &root, &rules));
    assert!(is_ignored(&root.join("target/file.o"), &root, &rules));
    assert!(!is_ignored(&root.join("src/main.rs"), &root, &rules));
}

#[test]
fn ignore_glob_pattern() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    fs::write(root.join(".rustygitignore"), "*.log\n").unwrap();

    fs::write(root.join("debug.log"), "log").unwrap();
    fs::write(root.join("info.txt"), "text").unwrap();

    let rules = parse_ignore_file(&root).unwrap();

    assert!(is_ignored(&root.join("debug.log"), &root, &rules));
    assert!(!is_ignored(&root.join("info.txt"), &root, &rules));
}

use rustygit::commands::{init, write_tree};

#[test]
fn ignored_files_do_not_change_tree_hash() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    init(&root).unwrap();

    fs::write(root.join(".rustygitignore"), "*.tmp\n").unwrap();
    fs::write(root.join("a.txt"), "hello").unwrap();
    fs::write(root.join("ignore.tmp"), "one").unwrap();

    let rules = parse_ignore_file(&root).unwrap();
    let hash1 = write_tree(&root, &root, &rules).unwrap();

    // Change ignored file
    fs::write(root.join("ignore.tmp"), "two").unwrap();

    let rules = parse_ignore_file(&root).unwrap();
    let hash2 = write_tree(&root, &root, &rules).unwrap();

    assert_eq!(hash1, hash2);
}

#[test]
fn gitignore_file_is_not_tracked() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();

    fs::write(root.join(".gitignore"), "foo\n").unwrap();

    let rules = parse_ignore_file(&root).unwrap();

    assert!(is_ignored(&root.join(".gitignore"), &root, &rules) == false);
}
