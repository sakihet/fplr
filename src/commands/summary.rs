use textplots::{Chart, Plot, Shape};

use crate::api::FplClient;
use crate::error::Result;

pub async fn handle_player_summary(player_id: u64, show_graph: bool) -> Result<()> {
    let summary = FplClient::fetch_player_summary(player_id).await?;
    let histories = summary.history;

    if show_graph {
        let points_data: Vec<(f32, f32)> = histories
            .iter()
            .map(|h| (h.round as f32, h.total_points as f32))
            .collect();

        if !points_data.is_empty() {
            println!("\nPoints per Gameweek:");
            Chart::new_with_y_range(120, 60, 1.0, points_data.len() as f32, 0.0, 20.0)
                .lineplot(&Shape::Lines(&points_data))
                .display();
        }
    } else {
        println!(
            "{:<3} {:<3} {:<4} {:<2} {:<2}",
            "GW", "Pts", "Min", "G", "A"
        );
        for history in histories.iter() {
            println!(
                "{:<3} {:<3} {:<4} {:<2} {:<2}",
                history.round,
                history.total_points,
                history.minutes,
                history.goals_scored,
                history.assists
            );
        }
    }

    Ok(())
}
