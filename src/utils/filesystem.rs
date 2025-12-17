use anyhow::{Result, bail};
use std::path::Path;

pub fn ensure_repo_exists(path: &Path) -> Result<()> {
    let mut paths = Vec::new();

    paths.push(path.join(".rustygit"));
    paths.push(path.join(".rustygit/objects"));
    paths.push(path.join(".rustygit/refs"));
    paths.push(path.join(".rustygit/refs/heads"));
    paths.push(path.join(".rustygit/HEAD"));

    for path in paths {
        if !path.exists() {
            bail!(
                "Could not find a Rusty Git repository in the specified path.\nPlease initialize a repository first.\n"
            );
        }
    }

    Ok(())
}
