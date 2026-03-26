use assert_cmd::Command;
use rustygit::commands;
use std::{fs, path::Path};
use tempfile::tempdir;

fn run_diff(repo_root: &Path) -> String {
    let assert = Command::cargo_bin("rustygit")
        .unwrap()
        .current_dir(repo_root)
        .arg("diff")
        .assert()
        .success();

    String::from_utf8_lossy(&assert.get_output().stdout).to_string()
}

#[test]
fn diff_no_changes_after_commit() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();

    commands::init(repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"line one\nline two\n").unwrap();
    commands::commit(repo_root, "initial".to_string(), &vec![]).unwrap();

    let output = run_diff(repo_root);

    assert!(output.trim().is_empty() || output.to_lowercase().contains("no changes"));
}

#[test]
fn diff_single_line_change_shows_one_deletion_and_insertion() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();

    commands::init(repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"line one\nline two\n").unwrap();
    commands::commit(repo_root, "initial".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"line one\nline changed\n").unwrap();

    let output = run_diff(repo_root);

    assert!(output.contains("Modified files:"));
    assert!(output.contains("a.txt"));
    assert_eq!(output.matches("\t\t-").count(), 1);
    assert_eq!(output.matches("\t\t+").count(), 1);
}

#[test]
fn diff_multiple_files_lists_both_files() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();

    commands::init(repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"alpha\n").unwrap();
    fs::write(repo_root.join("b.txt"), b"beta\n").unwrap();
    commands::commit(repo_root, "initial".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"alpha changed\n").unwrap();
    fs::write(repo_root.join("b.txt"), b"beta changed\n").unwrap();

    let output = run_diff(repo_root);

    assert!(output.contains("Modified files:"));
    assert!(output.contains("a.txt"));
    assert!(output.contains("b.txt"));
}

#[test]
fn delete_one_file_shows_deleted_file() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();

    commands::init(repo_root).unwrap();
    fs::write(repo_root.join("a.txt"), b"alpha\n").unwrap();
    commands::commit(repo_root, "initial".to_string(), &vec![]).unwrap();

    fs::remove_file(repo_root.join("a.txt")).unwrap();

    let output = run_diff(repo_root);

    assert!(output.contains("Deleted files:"));
    assert!(output.contains("a.txt"));
}
