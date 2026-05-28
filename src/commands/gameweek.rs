use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::formatters::format_datetime_local;

pub async fn handle_gameweek() -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    println!(
        "{:<id_w$} {:<name_w$} {:<status_w$} {:<time_w$}",
        "ID",
        "Name",
        "Status",
        "Deadline",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        status_w = WIDTH_STATUS,
        time_w = WIDTH_TIME,
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
            "{:<id_w$} {:<name_w$} {:<status_w$} {:<time_w$}",
            event.id,
            event.name,
            status,
            format_datetime_local(&event.deadline_time),
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            status_w = WIDTH_STATUS,
            time_w = WIDTH_TIME,
        );
    }

    Ok(())
}
