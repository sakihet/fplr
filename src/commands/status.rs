use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::utils::event_helpers::{find_current_event, find_next_event};
use crate::utils::formatters::format_datetime_local;

use crate::utils::constants::*;

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

    let fixtures = if let Some(current) = current_event {
        FplClient::fetch_fixtures_by_event(current.id as u32)
            .await
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    let total_fixtures = fixtures.len();
    let finished_fixtures = fixtures.iter().filter(|f| f.finished).count();
    let started_fixtures = fixtures
        .iter()
        .filter(|f| f.started == Some(true) && !f.finished)
        .count();

    if let (Some(current), Some(next)) = (current_event, next_event) {
        println!(
            "{:<gw_w$}  {:<avg_w$}  {:<pts_w$}  {:<max_w$}  {:<time_w$}",
            "GW",
            "Average",
            "Points",
            "Highest",
            "Next Deadline",
            gw_w = WIDTH_GW,
            avg_w = WIDTH_AVERAGE,
            pts_w = WIDTH_POINTS,
            max_w = WIDTH_HIGHEST,
            time_w = WIDTH_TIME,
        );
        println!(
            "{:<gw_w$}  {:<avg_w$}  {:<pts_w$}  {:<max_w$}  {:<time_w$}",
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
            format_datetime_local(&next.deadline_time),
            gw_w = WIDTH_GW,
            avg_w = WIDTH_AVERAGE,
            pts_w = WIDTH_POINTS,
            max_w = WIDTH_HIGHEST,
            time_w = WIDTH_TIME,
        );

        if total_fixtures > 0 {
            let in_progress_text = if started_fixtures > 0 {
                format!(" ({} In Progress)", started_fixtures)
            } else {
                "".to_string()
            };
            println!(
                "\nFixtures: {} / {} Finished{}",
                finished_fixtures, total_fixtures, in_progress_text
            );
        }
    } else {
        println!("No current gameweek found.");
    }

    Ok(())
}
