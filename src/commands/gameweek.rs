use crate::api::FplClient;
use crate::error::Result;
use crate::utils::formatters::format_datetime_local;

pub async fn handle_gameweek() -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    println!(
        "{:<4} {:<16} {:<12} {:<24}",
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
            "{:<4} {:<16} {:<12} {:<24}",
            event.id,
            event.name,
            status,
            format_datetime_local(&event.deadline_time)
        );
    }

    Ok(())
}
