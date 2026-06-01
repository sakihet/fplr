use textplots::{Chart, Plot, Shape};

use crate::api::FplClient;
use crate::error::Result;
use crate::models::PlayerHistory;

pub async fn handle_player_summary(
    player_id: u64,
    show_graph: bool,
    show_xg: bool,
    show_ict: bool,
    show_fpl: bool,
) -> Result<()> {
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
    } else if show_xg {
        print_xg(&histories);
    } else if show_ict {
        print_ict(&histories);
    } else if show_fpl {
        print_fpl(&histories);
    } else {
        print_default(&histories);
    }

    Ok(())
}

fn print_default(histories: &[PlayerHistory]) {
    println!(
        "{:<3} {:<3} {:<4} {:<2} {:<2}",
        "GW", "Pts", "Min", "G", "A"
    );
    for h in histories {
        println!(
            "{:<3} {:<3} {:<4} {:<2} {:<2}",
            h.round, h.total_points, h.minutes, h.goals_scored, h.assists
        );
    }
}

fn print_xg(histories: &[PlayerHistory]) {
    println!(
        "{:<3} {:<3} {:>5} {:>5} {:>5} {:>5}",
        "GW", "Pts", "xG", "xA", "xGI", "xGC"
    );
    for h in histories {
        println!(
            "{:<3} {:<3} {:>5} {:>5} {:>5} {:>5}",
            h.round,
            h.total_points,
            h.expected_goals,
            h.expected_assists,
            h.expected_goal_involvements,
            h.expected_goals_conceded,
        );
    }
}

fn print_ict(histories: &[PlayerHistory]) {
    println!(
        "{:<3} {:<3} {:>6} {:>6} {:>6} {:>6}",
        "GW", "Pts", "Inf", "Cre", "Thr", "ICT"
    );
    for h in histories {
        println!(
            "{:<3} {:<3} {:>6} {:>6} {:>6} {:>6}",
            h.round, h.total_points, h.influence, h.creativity, h.threat, h.ict_index,
        );
    }
}

fn print_fpl(histories: &[PlayerHistory]) {
    println!(
        "{:<3} {:<3} {:>4} {:>8} {:>6} {:>6}",
        "GW", "Pts", "Val", "Sel", "TrIn", "TrOut"
    );
    for h in histories {
        let val = format!("{:.1}", h.value as f64 / 10.0);
        println!(
            "{:<3} {:<3} {:>4} {:>8} {:>6} {:>6}",
            h.round, h.total_points, val, h.selected, h.transfers_in, h.transfers_out,
        );
    }
}
