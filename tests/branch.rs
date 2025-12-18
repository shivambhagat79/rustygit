use rustygit::commands;
use std::fs;
use tempfile::tempdir;

#[test]
fn branch_created() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // create a file
    fs::write(repo_root.join("a.txt"), b"one").unwrap();

    let commit_hash = commands::commit(&repo_root, String::from("First"), &vec![]).unwrap();

    commands::create_branch(&repo_root, "new-branch").unwrap();

    let branch_path = repo_root
        .join(".rustygit")
        .join("refs")
        .join("heads")
        .join("new-branch");
    let branch_content = fs::read_to_string(&branch_path).unwrap();

    assert!(branch_path.exists());
    assert_eq!(branch_content, commit_hash);

    let output = commands::create_branch(&repo_root, "new-branch");

    assert!(output.is_err());
}

#[test]
fn list_branches() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();
    std::env::set_current_dir(&repo_root).unwrap();

    // init repository
    commands::init(&repo_root).unwrap();
    fs::create_dir_all(repo_root.join(".git")).unwrap();

    // create a file
    fs::write(repo_root.join("a.txt"), b"one").unwrap();

    commands::commit(&repo_root, String::from("First"), &vec![]).unwrap();

    commands::create_branch(&repo_root, "branch-one").unwrap();
    commands::create_branch(&repo_root, "branch-two").unwrap();

    let output = commands::branch(&repo_root);

    assert!(output.is_ok());
}
