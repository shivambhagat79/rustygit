pub use chrono::Local;

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
