use crate::api::FplClient;
use crate::utils::formatters::format_datetime;

pub async fn handle_gameweek() {
    match FplClient::fetch_bootstrap_static().await {
        Ok(data) => {
            println!(
                "{:<4} {:<16} {:<12} {:<20}",
                "ID", "Name", "Status", "Deadline"
            );
            for event in data.events {
                let status = if event.is_current {
                    "Current"
                } else if event.is_next {
                    "Next"
                } else if event.finished {
                    "Finished"
                } else {
                    "Upcoming"
                };
                println!(
                    "{:<4} {:<16} {:<12} {:<20}",
                    event.id,
                    event.name,
                    status,
                    format_datetime(&event.deadline_time)
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
