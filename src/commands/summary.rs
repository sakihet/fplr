use owo_colors::OwoColorize;
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
    let (summary, bootstrap) = tokio::join!(
        FplClient::fetch_player_summary(player_id),
        FplClient::fetch_bootstrap_static()
    );
    let summary = summary?;
    let histories = summary.history;

    if let Ok(bs) = bootstrap
        && let Some(element) = bs.elements.iter().find(|e| e.id == player_id)
    {
        println!("{}", element.web_name);
    }

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

fn color_points(pts: i64) -> String {
    let s = format!("{:<3}", pts);
    if pts >= 10 {
        s.green().to_string()
    } else if pts <= 1 {
        s.red().to_string()
    } else {
        s
    }
}

fn print_default(histories: &[PlayerHistory]) {
    println!(
        "{:<3} {:<3} {:<4} {:<2} {:<2}",
        "GW", "Pts", "Min", "G", "A"
    );
    for h in histories {
        println!(
            "{:<3} {} {:<4} {:<2} {:<2}",
            h.round,
            color_points(h.total_points),
            h.minutes,
            h.goals_scored,
            h.assists
        );
    }
    if !histories.is_empty() {
        let n = histories.len() as f64;
        let total_pts: i64 = histories.iter().map(|h| h.total_points).sum();
        let total_min: u64 = histories.iter().map(|h| h.minutes).sum();
        let total_g: u64 = histories.iter().map(|h| h.goals_scored).sum();
        let total_a: u64 = histories.iter().map(|h| h.assists).sum();
        println!("{:-<18}", "");
        println!(
            "{:<3} {:<3} {:<4} {:<2} {:<2}",
            "Tot", total_pts, total_min, total_g, total_a
        );
        println!(
            "{:<3} {:<3} {:<4} {:<2} {:<2}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            format!("{:.0}", total_min as f64 / n),
            format!("{:.1}", total_g as f64 / n),
            format!("{:.1}", total_a as f64 / n),
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
            "{:<3} {} {:>5} {:>5} {:>5} {:>5}",
            h.round,
            color_points(h.total_points),
            h.expected_goals,
            h.expected_assists,
            h.expected_goal_involvements,
            h.expected_goals_conceded,
        );
    }
    if !histories.is_empty() {
        let n = histories.len() as f64;
        let total_pts: i64 = histories.iter().map(|h| h.total_points).sum();
        let sum_xg: f64 = histories
            .iter()
            .filter_map(|h| h.expected_goals.parse::<f64>().ok())
            .sum();
        let sum_xa: f64 = histories
            .iter()
            .filter_map(|h| h.expected_assists.parse::<f64>().ok())
            .sum();
        let sum_xgi: f64 = histories
            .iter()
            .filter_map(|h| h.expected_goal_involvements.parse::<f64>().ok())
            .sum();
        let sum_xgc: f64 = histories
            .iter()
            .filter_map(|h| h.expected_goals_conceded.parse::<f64>().ok())
            .sum();
        println!("{:-<30}", "");
        println!(
            "{:<3} {:<3} {:>5.2} {:>5.2} {:>5.2} {:>5.2}",
            "Tot", total_pts, sum_xg, sum_xa, sum_xgi, sum_xgc,
        );
        println!(
            "{:<3} {:<3} {:>5.2} {:>5.2} {:>5.2} {:>5.2}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            sum_xg / n,
            sum_xa / n,
            sum_xgi / n,
            sum_xgc / n,
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
            "{:<3} {} {:>6} {:>6} {:>6} {:>6}",
            h.round,
            color_points(h.total_points),
            h.influence,
            h.creativity,
            h.threat,
            h.ict_index,
        );
    }
    if !histories.is_empty() {
        let n = histories.len() as f64;
        let total_pts: i64 = histories.iter().map(|h| h.total_points).sum();
        let sum_inf: f64 = histories
            .iter()
            .filter_map(|h| h.influence.parse::<f64>().ok())
            .sum();
        let sum_cre: f64 = histories
            .iter()
            .filter_map(|h| h.creativity.parse::<f64>().ok())
            .sum();
        let sum_thr: f64 = histories
            .iter()
            .filter_map(|h| h.threat.parse::<f64>().ok())
            .sum();
        let sum_ict: f64 = histories
            .iter()
            .filter_map(|h| h.ict_index.parse::<f64>().ok())
            .sum();
        println!("{:-<36}", "");
        println!(
            "{:<3} {:<3} {:>6.1} {:>6.1} {:>6.1} {:>6.1}",
            "Tot", total_pts, sum_inf, sum_cre, sum_thr, sum_ict,
        );
        println!(
            "{:<3} {:<3} {:>6.1} {:>6.1} {:>6.1} {:>6.1}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            sum_inf / n,
            sum_cre / n,
            sum_thr / n,
            sum_ict / n,
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
            "{:<3} {} {:>4} {:>8} {:>6} {:>6}",
            h.round,
            color_points(h.total_points),
            val,
            h.selected,
            h.transfers_in,
            h.transfers_out,
        );
    }
    if !histories.is_empty() {
        let n = histories.len() as f64;
        let total_pts: i64 = histories.iter().map(|h| h.total_points).sum();
        let total_tr_in: u64 = histories.iter().map(|h| h.transfers_in).sum();
        let total_tr_out: u64 = histories.iter().map(|h| h.transfers_out).sum();
        let avg_val = histories.iter().map(|h| h.value).sum::<u64>() as f64 / n / 10.0;
        let avg_sel = histories.iter().map(|h| h.selected).sum::<u64>() as f64 / n;
        println!("{:-<38}", "");
        println!(
            "{:<3} {:<3} {:>4} {:>8} {:>6} {:>6}",
            "Tot", total_pts, "-", "-", total_tr_in, total_tr_out,
        );
        println!(
            "{:<3} {:<3} {:>4.1} {:>8.0} {:>6} {:>6}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            avg_val,
            avg_sel,
            "-",
            "-",
        );
    }
}
