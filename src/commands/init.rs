use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

pub fn init(path: &Path) -> Result<()> {
    let rusty_git_dir = path.join(".rustygit");

    if rusty_git_dir.exists() {
        bail!("Repository already exists at {}", path.display());
    }

    // Create directory structure
    fs::create_dir_all(rusty_git_dir.join("objects"))?;
    fs::create_dir_all(rusty_git_dir.join("refs").join("heads"))?;
    fs::create_dir_all(rusty_git_dir.join("refs").join("tags"))?;

    // Create HEAD file
    let head_contents = "ref: refs/heads/main\n";
    fs::write(rusty_git_dir.join("HEAD"), head_contents)?;

    Ok(())
}
