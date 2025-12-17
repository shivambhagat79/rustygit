use anyhow::{Result, bail};

use crate::utils;

/// Formats a single commit object (raw on-disk contents) into a git-like log entry.
///
/// Expected object structure (similar to git):
/// - Header: "commit <size>\0"
/// - Then lines like:
///   - tree <hash>
///   - parent <hash>
///   - author Name <email> <epoch> <tz>
///   - committer ...
/// - Blank line
/// - Commit message (possibly multi-line)
///
/// Output format (plain text example):
/// ```text
/// commit <hash>
/// Author: Name <email>
/// Date:   Wed Dec  6 12:34:56 2025 +0530
///
///     message line 1
///     message line 2
/// ```
pub fn format_commit_history(commit_data: &str, hash: &str) -> Result<String> {
    let nul_idx = commit_data
        .find('\0')
        .ok_or_else(|| anyhow::anyhow!("Commit object missing NUL separator"))?;

    let (metadata, rest) = commit_data.split_at(nul_idx);
    if !metadata.trim().starts_with("commit") {
        bail!("Error parsing commit object.\nHash: {}", hash);
    }

    // Skip the NUL; remaining content is headers + blank line + message.
    let data = &rest[1..];

    let mut author: Option<String> = None;
    let mut date: Option<String> = None;

    let mut out = String::new();
    out.push_str(&format!("commit {}\n", hash));

    let mut in_message = false;

    for raw_line in data.lines() {
        let line = raw_line.trim_end_matches('\r');

        if !in_message {
            if line.is_empty() {
                if let Some(a) = author.as_deref() {
                    out.push_str(&format!("Author: {}\n", a));
                }
                if let Some(d) = date.as_deref() {
                    out.push_str(&format!("Date:   {}\n", d));
                }
                out.push('\n');
                in_message = true;
                continue;
            }

            if line.starts_with("tree ")
                || line.starts_with("parent ")
                || line.starts_with("committer ")
            {
                continue;
            }

            if let Some(rest) = line.strip_prefix("author ") {
                // rest: "Name <email> <epoch> <tz>"
                let parts: Vec<&str> = rest.split_whitespace().collect();

                // If it matches the expected shape, keep "Name <email>" and parse date.
                if parts.len() >= 4 {
                    author = Some(parts[..parts.len() - 2].join(" "));

                    let epoch_str = parts[parts.len() - 2];
                    let tz = parts[parts.len() - 1];

                    if let Ok(epoch) = epoch_str.parse::<i64>() {
                        date = utils::format_commit_date(epoch, tz);
                    }
                } else {
                    // Fallback: keep entire author line as-is.
                    author = Some(rest.to_string());
                }

                continue;
            }

            // Ignore unknown headers for now.
            continue;
        }

        // Commit message (indented by 4 spaces like `git log`)
        if line.is_empty() {
            out.push('\n');
        } else {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
    }

    Ok(out)
}
