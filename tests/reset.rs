use rustygit::{commands, utils};
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

fn tree_map_for_commit(repo_root: &Path, commit_hash: &str) -> HashMap<std::path::PathBuf, String> {
    let commit_content = utils::parse_commit(repo_root, commit_hash).unwrap();
    let n_idx = commit_content.find('\n').unwrap();
    let tree_hash = &commit_content[5..n_idx];

    let mut map = HashMap::new();
    utils::get_tree_files_map(repo_root, Path::new(""), tree_hash, &mut map).unwrap();
    map
}

#[test]
fn soft_reset_moves_head_and_keeps_index() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();

    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    let commit_a = commands::commit(&repo_root, "A".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    let commit_b = commands::commit(&repo_root, "B".to_string(), &vec![]).unwrap();

    commands::reset(&repo_root, &commit_a, true).unwrap();

    let head_after = utils::get_current_commit_hash(&repo_root).unwrap().unwrap();
    assert_eq!(head_after, commit_a);

    let index_map = utils::read_index_map(&repo_root).unwrap();
    let tree_b = tree_map_for_commit(&repo_root, &commit_b);
    assert_eq!(index_map, tree_b);

    let work_content = fs::read_to_string(repo_root.join("a.txt")).unwrap();
    assert_eq!(work_content, "two");
}

#[test]
fn mixed_reset_moves_head_and_resets_index_only() {
    let dir = tempdir().unwrap();
    let repo_root = dir.path().canonicalize().unwrap();

    commands::init(&repo_root).unwrap();

    fs::write(repo_root.join("a.txt"), b"one").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    let commit_a = commands::commit(&repo_root, "A".to_string(), &vec![]).unwrap();

    fs::write(repo_root.join("a.txt"), b"two").unwrap();
    commands::add(&repo_root, &repo_root.join("a.txt")).unwrap();
    let _commit_b = commands::commit(&repo_root, "B".to_string(), &vec![]).unwrap();

    commands::reset(&repo_root, &commit_a, false).unwrap();

    let head_after = utils::get_current_commit_hash(&repo_root).unwrap().unwrap();
    assert_eq!(head_after, commit_a);

    let index_map = utils::read_index_map(&repo_root).unwrap();
    let tree_a = tree_map_for_commit(&repo_root, &commit_a);
    assert_eq!(index_map, tree_a);

    let work_content = fs::read_to_string(repo_root.join("a.txt")).unwrap();
    assert_eq!(work_content, "two");
}
