use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;

pub fn format_datetime(datetime_str: &str) -> String {
    let dt = datetime_str.parse::<DateTime<Utc>>().unwrap();
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
}

pub fn difficulty_to_stars(difficulty: u8) -> String {
    "*".repeat(difficulty as usize)
}

pub fn colorize_text_by_difficulty(text: &str, difficulty: u8) -> String {
    match difficulty {
        1 | 2 => text.green().to_string(),
        3 => text.to_string(),
        4 | 5 => text.red().to_string(),
        _ => text.to_string(),
    }
}
