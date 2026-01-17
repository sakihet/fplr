use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;

pub fn format_datetime(datetime_str: &str) -> String {
    let dt = datetime_str.parse::<DateTime<Utc>>().unwrap();
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
}

pub fn difficulty_to_stars(difficulty: u8) -> String {
    "*".repeat(difficulty as usize)
}

pub fn colorize_difficulty(difficulty: u8) -> String {
    let padded = format!("{:<5}", difficulty);
    match difficulty {
        1 | 2 => padded.green().to_string(),
        3 => padded,
        4 | 5 => padded.red().to_string(),
        _ => padded,
    }
}
