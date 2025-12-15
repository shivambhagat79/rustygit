use rustygit::repo::init;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_init_command() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    init(path).unwrap();

    let git_dir = path.join(".rustygit");

    assert!(git_dir.exists());
    assert!(git_dir.join("objects").exists());
    assert!(git_dir.join("refs").join("heads").exists());
    assert!(git_dir.join("refs").join("tags").exists());

    let head = fs::read_to_string(git_dir.join("HEAD")).unwrap();
    assert_eq!(head, "ref: refs/heads/main\n");
}
