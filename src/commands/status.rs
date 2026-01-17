use crate::api::FplClient;
use crate::utils::formatters::format_datetime;

pub async fn handle_status() {
    match FplClient::fetch_bootstrap_static().await {
        Ok(data) => {
            let current_event = data.events.iter().find(|e| e.is_current);
            let next_event = data.events.iter().find(|e| e.is_next);

            if let (Some(current), Some(next)) = (current_event, next_event) {
                println!(
                    "{:<8} {:<8} {:<8} {:<20}",
                    "Gameweek", "Average", "Highest", "Next Deadline"
                );
                println!(
                    "{:<8} {:<8} {:<8} {:<20}",
                    current.id,
                    current
                        .average_entry_score
                        .map(|s| s.to_string())
                        .unwrap_or("-".to_string()),
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
