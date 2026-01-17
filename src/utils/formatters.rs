use chrono::{DateTime, Utc};

pub fn format_datetime(datetime_str: &str) -> String {
    let dt = datetime_str.parse::<DateTime<Utc>>().unwrap();
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
}

pub fn difficulty_to_stars(difficulty: u8) -> String {
    "*".repeat(difficulty as usize)
}

#[allow(dead_code)]
pub fn difficulty_colored(_difficulty: u8, text: &str) -> String {
    text.to_string()
}
