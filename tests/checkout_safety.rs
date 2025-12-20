use rustygit::commands;
use std::fs;
use tempfile::tempdir;

#[test]
fn untracked_file_blocks_checkout() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();
    std::env::set_current_dir(&repo_root).unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // commit with a.txt
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    let hash_first = commands::commit(&repo_root, "First".to_string(), &vec![]).unwrap();

    // second commit removes a.txt
    fs::remove_file(repo_root.join("a.txt")).unwrap();
    commands::commit(&repo_root, "Second".to_string(), &vec![]).unwrap();

    // untracked file at path that target wants to write
    fs::write(repo_root.join("a.txt"), b"untracked").unwrap();

    // checkout commit that *has* a.txt
    let result = commands::checkout(&repo_root, &hash_first);
    assert!(result.is_err());
}

#[test]
fn modified_tracked_file_blocks_checkout() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();
    std::env::set_current_dir(&repo_root).unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // initial commit
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    let hash_first = commands::commit(&repo_root, "First".to_string(), &vec![]).unwrap();

    // second commit changes the file
    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::commit(&repo_root, "Second".to_string(), &vec![]).unwrap();

    // modify tracked file again (dirty working tree)
    fs::write(repo_root.join("a.txt"), b"dirty").unwrap();

    let result = commands::checkout(&repo_root, &hash_first);
    assert!(result.is_err());
}

#[test]
fn modified_tracked_file_blocks_checkout_on_delete() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();
    std::env::set_current_dir(&repo_root).unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // commit with file
    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    let hash_first = commands::commit(&repo_root, "First".to_string(), &vec![]).unwrap();

    // commit that removes the file
    fs::remove_file(repo_root.join("a.txt")).unwrap();
    let hash_second = commands::commit(&repo_root, "Second".to_string(), &vec![]).unwrap();

    // checkout back to first
    commands::checkout(&repo_root, &hash_first).unwrap();

    // modify tracked file
    fs::write(repo_root.join("a.txt"), b"dirty").unwrap();

    // checkout commit that deletes the file
    let result = commands::checkout(&repo_root, &hash_second);
    assert!(result.is_err());
}

#[test]
fn unmodified_tracked_file_allows_checkout() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();
    std::env::set_current_dir(&repo_root).unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    let hash_first = commands::commit(&repo_root, "First".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::commit(&repo_root, "Second".to_string(), &vec![]).unwrap();

    let result = commands::checkout(&repo_root, &hash_first);
    assert!(result.is_ok());

    let content = fs::read_to_string(repo_root.join("a.txt")).unwrap();
    assert_eq!(content, "one");
}

#[test]
fn untracked_file_not_overwritten_is_allowed() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();
    std::env::set_current_dir(&repo_root).unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    let hash_first = commands::commit(&repo_root, "First".to_string(), &vec![]).unwrap();

    // untracked file that target does not touch
    fs::write(repo_root.join("b.txt"), b"untracked").unwrap();

    let result = commands::checkout(&repo_root, &hash_first);
    assert!(result.is_ok());
}

#[test]
fn modified_file_allowed_if_target_keeps_same_version() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path();
    std::env::set_current_dir(&repo_root).unwrap();

    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    let hash_first = commands::commit(&repo_root, "First".to_string(), &vec![]).unwrap();

    // checkout same commit (no tree change)
    fs::write(repo_root.join("a.txt"), b"dirty").unwrap();

    let result = commands::checkout(&repo_root, &hash_first);
    assert!(result.is_ok());
}
