use std::fmt::Display;

use crate::api::FplClient;
use crate::config::Config;
use crate::utils::formatters::format_datetime;

pub async fn handle_status() {
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Error loading config: {}", e);
        Config::default()
    });

    let manager_id_str = config.user.and_then(|u| u.manager_id);
    let manager_id = if let Some(id_str) = manager_id_str {
        match id_str.parse::<u64>() {
            Ok(id) => Some(id),
            Err(_) => {
                eprintln!("Warning: Invalid manager ID in config. Ignoring.");
                None
            }
        }
    } else {
        None
    };

    match FplClient::fetch_bootstrap_static().await {
        Ok(data) => {
            let current_event = data.events.iter().find(|e| e.is_current);
            let next_event = data.events.iter().find(|e| e.is_next);

            let my_team_score: Box<dyn Display> = if let Some(mid) = manager_id {
                if let Some(current) = current_event {
                    match FplClient::fetch_manager_picks(mid, current.id as u32).await {
                        Ok(picks) => Box::new(picks.entry_history.points),
                        Err(e) => {
                            eprintln!("Error fetching manager picks: {}", e);
                            Box::new("-".to_string())
                        }
                    }
                } else {
                    Box::new("-".to_string())
                }
            } else {
                Box::new("-".to_string())
            };

            if let (Some(current), Some(next)) = (current_event, next_event) {
                println!(
                    "{:<8} {:<8} {:<8} {:<8} {:<15}",
                    "GW", "Average", "Points", "Highest", "Next Deadline"
                );
                println!(
                    "{:<8} {:<8} {:<8} {:<8} {:<15}",
                    current.id,
                    current
                        .average_entry_score
                        .map(|s| s.to_string())
                        .unwrap_or("-".to_string()),
                    my_team_score,
                    current
                        .highest_score
                        .map(|s| s.to_string())
                        .unwrap_or("-".to_string()),
                    format_datetime(&next.deadline_time),
                );
            } else {
                println!("No current gameweek found.");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
