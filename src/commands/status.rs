use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::utils::event_helpers::{find_current_event, find_next_event};
use crate::utils::formatters::format_datetime;

pub async fn handle_status() -> Result<()> {
    let config = Config::load().unwrap_or_default();
    let manager_id = config.get_manager_id().ok();

    let data = FplClient::fetch_bootstrap_static().await?;
    let current_event = find_current_event(&data.events);
    let next_event = find_next_event(&data.events);

    let my_team_score: String = if let Some(mid) = manager_id {
        if let Some(current) = current_event {
            match FplClient::fetch_manager_picks(mid, current.id as u32).await {
                Ok(picks) => picks.entry_history.points.to_string(),
                Err(_) => "-".to_string(),
            }
        } else {
            "-".to_string()
        }
    } else {
        "-".to_string()
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

    Ok(())
}
