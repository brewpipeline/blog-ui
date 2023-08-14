use chrono::{Local, TimeZone, Utc, Locale};

pub fn format(milliseconds: u64) -> String {
    let dt = Utc
        .timestamp_opt((milliseconds as i64) / 1000, 0)
        .unwrap()
        .with_timezone(&Local);
    dt.format_localized("%H:%M | %e %B %Y", Locale::ru_RU).to_string()
}
