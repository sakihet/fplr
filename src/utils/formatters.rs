use chrono::{DateTime, Local, Utc};
use chrono_tz::Tz;
use owo_colors::OwoColorize;

pub fn format_datetime(datetime_str: &str) -> String {
    let dt = datetime_str.parse::<DateTime<Utc>>().unwrap();
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
}

pub fn format_datetime_local(datetime_str: &str) -> String {
    let dt = datetime_str.parse::<DateTime<Utc>>().unwrap();

    // Try to get system timezone name and use chrono-tz for proper abbreviation
    if let Some(tz) = get_system_timezone() {
        let local_dt = dt.with_timezone(&tz);
        return local_dt.format("%Y-%m-%d %H:%M %Z").to_string();
    }

    // Fallback to offset format
    let local_dt = dt.with_timezone(&Local);
    local_dt.format("%Y-%m-%d %H:%M %:z").to_string()
}

fn get_system_timezone() -> Option<Tz> {
    // Try TZ environment variable first
    if let Ok(tz_str) = std::env::var("TZ")
        && let Ok(tz) = tz_str.parse::<Tz>()
    {
        return Some(tz);
    }

    // On macOS/Linux, try to read from /etc/localtime symlink
    #[cfg(unix)]
    {
        if let Ok(link) = std::fs::read_link("/etc/localtime") {
            let path_str = link.to_string_lossy();
            // Extract timezone from path like /var/db/timezone/zoneinfo/Asia/Tokyo
            if let Some(pos) = path_str.find("zoneinfo/") {
                let tz_name = &path_str[pos + 9..];
                if let Ok(tz) = tz_name.parse::<Tz>() {
                    return Some(tz);
                }
            }
        }
    }

    None
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

// ============================================
// Color helpers for common output patterns
// ============================================

/// Color a trend indicator: ↑ green, ↓ red, → default
pub fn color_trend(trend: &str) -> String {
    match trend {
        "↑" => trend.green().to_string(),
        "↓" => trend.red().to_string(),
        _ => trend.to_string(),
    }
}

/// Color a match result: W green, L red, D yellow
pub fn color_form_result(result: char) -> String {
    match result {
        'W' => result.to_string().green().to_string(),
        'L' => result.to_string().red().to_string(),
        'D' => result.to_string().yellow().to_string(),
        _ => result.to_string(),
    }
}

/// Color a value based on comparison with baseline: above=green, below=red, equal=default
pub fn color_by_comparison(value: i64, baseline: i64) -> String {
    let value_str = format!("{:>5}", value);
    if value > baseline {
        value_str.green().to_string()
    } else if value < baseline {
        value_str.red().to_string()
    } else {
        value_str
    }
}

/// Color league position: CL (1-4) green, EL (5-6) cyan, Relegation (18-20) red
pub fn color_league_position(pos: usize, width: usize) -> String {
    let pos_str = format!("{:<width$}", pos, width = width);
    match pos {
        1..=4 => pos_str.green().to_string(), // Champions League
        5..=6 => pos_str.cyan().to_string(),  // Europa League
        18..=20 => pos_str.red().to_string(), // Relegation
        _ => pos_str,
    }
}

// ============================================
// Number formatting helpers
// ============================================

/// Format a number with sign (+/-) and optional padding
pub fn format_signed_number(n: i64) -> String {
    if n > 0 {
        format!("+{}", n)
    } else {
        n.to_string()
    }
}

/// Format and colorize chance of playing percentage
pub fn format_chance_of_playing(chance: Option<u64>, news: &str) -> String {
    let (chance_val, chance_str) = match chance {
        Some(c) => (c, format!("{}%", c)),
        None => {
            if news.is_empty() {
                (100, "100%".to_string())
            } else {
                (0, "0%".to_string())
            }
        }
    };

    let padded = format!("{:>5}", chance_str);
    match chance_val {
        100 => padded.default_color().to_string(),
        75 => padded.yellow().to_string(),
        50 => padded.bright_yellow().to_string(),
        _ => padded.red().to_string(),
    }
}
