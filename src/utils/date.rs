use chrono::{Local, TimeZone, Utc};

pub fn format(milliseconds: u64) -> String {
    let dt = Utc
        .timestamp_opt((milliseconds as i64) / 1000, 0)
        .unwrap()
        .with_timezone(&Local);
    dt.format("%b %e %H:%M %Y").to_string()
}
