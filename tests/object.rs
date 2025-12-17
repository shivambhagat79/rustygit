use std::fs;
use tempfile::tempdir;

#[test]
fn same_content_produces_same_hash() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("file.txt");

    fs::write(&file, b"hello").unwrap();

    let hash1 = capture_hash(&file);
    let hash2 = capture_hash(&file);

    assert_eq!(hash1, hash2);
}

fn capture_hash(path: &std::path::Path) -> String {
    use std::process::Command;

    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "hash-object",
            path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn object_is_written_to_disk() {
    let dir = tempdir().unwrap();
    std::env::set_current_dir(&dir).unwrap();

    fs::create_dir_all(dir.path().join(".rustygit/objects")).unwrap();

    let file = dir.path().join("data.txt");
    fs::write(&file, b"content").unwrap();

    let hash = rustygit::commands::hash_object(&file).unwrap();

    let (d, f) = hash.split_at(2);
    let object_path = dir.path().join(".rustygit").join("objects").join(d).join(f);

    assert!(object_path.exists());
}
