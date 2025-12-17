pub use chrono::Local;

/// Returns the current time as `(unix_timestamp, timezone_offset)` where
/// the timezone offset is formatted like `+0530` or `-0700`.
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

/// Formats a commit date in a git-like style from `(unix_timestamp, timezone_offset)`.
///
/// - `epoch_secs` is seconds since Unix epoch.
/// - `tz` is formatted like `+0530` or `-0700`.
///
/// Returns something like: `Wed Dec  6 12:34:56 2025 +0530`.
pub fn format_commit_date(epoch_secs: i64, tz: &str) -> Option<String> {
    let utc_dt = chrono::DateTime::from_timestamp(epoch_secs, 0)?;

    // Parse timezone offset like +0530 / -0700
    let offset_minutes = if tz.len() == 5 {
        let sign = &tz[0..1];
        let hh = &tz[1..3];
        let mm = &tz[3..5];

        match (hh.parse::<i32>(), mm.parse::<i32>()) {
            (Ok(hh), Ok(mm)) => {
                let total = hh * 60 + mm;
                if sign == "-" { -total } else { total }
            }
            _ => 0,
        }
    } else {
        0
    };

    let offset = chrono::FixedOffset::east_opt(offset_minutes * 60)?;
    Some(
        utc_dt
            .with_timezone(&offset)
            .format("%a %b %e %H:%M:%S %Y %z")
            .to_string(),
    )
}
