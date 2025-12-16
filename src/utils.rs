use anyhow::{Result, bail};
use chrono::Local;
use std::path::Path;

pub fn hex_to_bytes(hex: &str) -> [u8; 20] {
    let mut bytes = [0u8; 20];
    for i in 0..20 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
    }
    bytes
}

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

pub fn get_time() -> (i64, String) {
    let now = Local::now();

    let timestamp = now.timestamp();
    let offset_secs = now.offset().local_minus_utc();

    let sign = if offset_secs >= 0 { '+' } else { '-' };
    let offset_secs = offset_secs.abs();

    let hours = offset_secs / 3600;
    let minutes = (offset_secs % 3600) / 60;

    let timezone = format!("{}{:02}{:02}", sign, hours, minutes);

    (timestamp, timezone)
}
