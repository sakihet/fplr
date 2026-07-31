use owo_colors::OwoColorize;
use textplots::{Chart, Plot, Shape};

use crate::api::FplClient;
use crate::error::Result;
use crate::models::PlayerHistory;
use crate::utils::constants::{
    WIDTH_AVAIL, WIDTH_GW, WIDTH_PTS, WIDTH_STAT, WIDTH_STAT_SMALL, WIDTH_STAT_WIDE,
};

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
    let s = format!("{:<width$}", pts, width = WIDTH_PTS);
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
        "{:<gw_w$} {:<gw_w$} {:<min_w$} {:<g_w$} {:<g_w$}",
        "GW",
        "Pts",
        "Min",
        "G",
        "A",
        gw_w = WIDTH_PTS,
        min_w = WIDTH_STAT_SMALL,
        g_w = WIDTH_GW,
    );
    for h in histories {
        println!(
            "{:<gw_w$} {} {:<min_w$} {:<g_w$} {:<g_w$}",
            h.round,
            color_points(h.total_points),
            h.minutes,
            h.goals_scored,
            h.assists,
            gw_w = WIDTH_PTS,
            min_w = WIDTH_STAT_SMALL,
            g_w = WIDTH_GW,
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
            "{:<gw_w$} {:<gw_w$} {:<min_w$} {:<g_w$} {:<g_w$}",
            "Tot",
            total_pts,
            total_min,
            total_g,
            total_a,
            gw_w = WIDTH_PTS,
            min_w = WIDTH_STAT_SMALL,
            g_w = WIDTH_GW,
        );
        println!(
            "{:<gw_w$} {:<gw_w$} {:<min_w$} {:<g_w$} {:<g_w$}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            format!("{:.0}", total_min as f64 / n),
            format!("{:.1}", total_g as f64 / n),
            format!("{:.1}", total_a as f64 / n),
            gw_w = WIDTH_PTS,
            min_w = WIDTH_STAT_SMALL,
            g_w = WIDTH_GW,
        );
    }
}

fn print_xg(histories: &[PlayerHistory]) {
    println!(
        "{:<gw_w$} {:<gw_w$} {:>x_w$} {:>x_w$} {:>x_w$} {:>x_w$}",
        "GW",
        "Pts",
        "xG",
        "xA",
        "xGI",
        "xGC",
        gw_w = WIDTH_PTS,
        x_w = WIDTH_AVAIL,
    );
    for h in histories {
        println!(
            "{:<gw_w$} {} {:>x_w$} {:>x_w$} {:>x_w$} {:>x_w$}",
            h.round,
            color_points(h.total_points),
            h.expected_goals,
            h.expected_assists,
            h.expected_goal_involvements,
            h.expected_goals_conceded,
            gw_w = WIDTH_PTS,
            x_w = WIDTH_AVAIL,
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
            "{:<gw_w$} {:<gw_w$} {:>x_w$.2} {:>x_w$.2} {:>x_w$.2} {:>x_w$.2}",
            "Tot",
            total_pts,
            sum_xg,
            sum_xa,
            sum_xgi,
            sum_xgc,
            gw_w = WIDTH_PTS,
            x_w = WIDTH_AVAIL,
        );
        println!(
            "{:<gw_w$} {:<gw_w$} {:>x_w$.2} {:>x_w$.2} {:>x_w$.2} {:>x_w$.2}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            sum_xg / n,
            sum_xa / n,
            sum_xgi / n,
            sum_xgc / n,
            gw_w = WIDTH_PTS,
            x_w = WIDTH_AVAIL,
        );
    }
}

fn print_ict(histories: &[PlayerHistory]) {
    println!(
        "{:<gw_w$} {:<gw_w$} {:>s_w$} {:>s_w$} {:>s_w$} {:>s_w$}",
        "GW",
        "Pts",
        "Inf",
        "Cre",
        "Thr",
        "ICT",
        gw_w = WIDTH_PTS,
        s_w = WIDTH_STAT,
    );
    for h in histories {
        println!(
            "{:<gw_w$} {} {:>s_w$} {:>s_w$} {:>s_w$} {:>s_w$}",
            h.round,
            color_points(h.total_points),
            h.influence,
            h.creativity,
            h.threat,
            h.ict_index,
            gw_w = WIDTH_PTS,
            s_w = WIDTH_STAT,
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
            "{:<gw_w$} {:<gw_w$} {:>s_w$.1} {:>s_w$.1} {:>s_w$.1} {:>s_w$.1}",
            "Tot",
            total_pts,
            sum_inf,
            sum_cre,
            sum_thr,
            sum_ict,
            gw_w = WIDTH_PTS,
            s_w = WIDTH_STAT,
        );
        println!(
            "{:<gw_w$} {:<gw_w$} {:>s_w$.1} {:>s_w$.1} {:>s_w$.1} {:>s_w$.1}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            sum_inf / n,
            sum_cre / n,
            sum_thr / n,
            sum_ict / n,
            gw_w = WIDTH_PTS,
            s_w = WIDTH_STAT,
        );
    }
}

fn print_fpl(histories: &[PlayerHistory]) {
    println!(
        "{:<gw_w$} {:<gw_w$} {:>val_w$} {:>sel_w$} {:>tr_w$} {:>tr_w$}",
        "GW",
        "Pts",
        "Val",
        "Sel",
        "TrIn",
        "TrOut",
        gw_w = WIDTH_PTS,
        val_w = WIDTH_STAT_SMALL,
        sel_w = WIDTH_STAT_WIDE,
        tr_w = WIDTH_STAT,
    );
    for h in histories {
        let val = format!("{:.1}", h.value as f64 / 10.0);
        println!(
            "{:<gw_w$} {} {:>val_w$} {:>sel_w$} {:>tr_w$} {:>tr_w$}",
            h.round,
            color_points(h.total_points),
            val,
            h.selected,
            h.transfers_in,
            h.transfers_out,
            gw_w = WIDTH_PTS,
            val_w = WIDTH_STAT_SMALL,
            sel_w = WIDTH_STAT_WIDE,
            tr_w = WIDTH_STAT,
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
            "{:<gw_w$} {:<gw_w$} {:>val_w$} {:>sel_w$} {:>tr_w$} {:>tr_w$}",
            "Tot",
            total_pts,
            "-",
            "-",
            total_tr_in,
            total_tr_out,
            gw_w = WIDTH_PTS,
            val_w = WIDTH_STAT_SMALL,
            sel_w = WIDTH_STAT_WIDE,
            tr_w = WIDTH_STAT,
        );
        println!(
            "{:<gw_w$} {:<gw_w$} {:>val_w$.1} {:>sel_w$.0} {:>tr_w$} {:>tr_w$}",
            "Avg",
            format!("{:.1}", total_pts as f64 / n),
            avg_val,
            avg_sel,
            "-",
            "-",
            gw_w = WIDTH_PTS,
            val_w = WIDTH_STAT_SMALL,
            sel_w = WIDTH_STAT_WIDE,
            tr_w = WIDTH_STAT,
        );
    }
}
