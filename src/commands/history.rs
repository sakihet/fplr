use std::collections::HashMap;

use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::formatters::{color_by_comparison, format_compact_number};
use clap::Args;

#[derive(Debug, Args)]
pub struct HistoryArgs {
    /// Manager ID (uses configured ID if not provided)
    #[arg(short, long)]
    manager_id: Option<u64>,
}

pub async fn handle_history(args: HistoryArgs) -> Result<()> {
    // 1. Determine Manager ID
    let manager_id = if let Some(id) = args.manager_id {
        id
    } else {
        Config::load()?.get_manager_id()?
    };

    // 2. Fetch Manager History and Bootstrap Static (for avg/max scores)
    let (history, bootstrap) = tokio::join!(
        FplClient::fetch_manager_history(manager_id),
        FplClient::fetch_bootstrap_static()
    );

    let history = history?;

    // Build a map of event_id -> (average_score, highest_score)
    let event_scores: HashMap<u64, (Option<u64>, Option<u64>)> = match bootstrap {
        Ok(data) => data
            .events
            .into_iter()
            .map(|e| (e.id, (e.average_entry_score, e.highest_score)))
            .collect(),
        Err(_) => HashMap::new(),
    };

    if history.current.is_empty() {
        println!("No history data available for Manager ID: {}", manager_id);
        return Ok(());
    }

    // 3. Display Current Season History
    println!("\nManager History (ID: {})", manager_id);
    println!("=== Current Season ===\n");

    println!(
        "{:<gw_w$} {:>pts_w$} {:>avg_w$} {:>max_w$} {:>total_w$} {:>rank_w$} {:>rank_w$} {:>trn_w$} {:>bnch_w$} {:>val_w$}",
        "GW",
        "Pts",
        "Avg",
        "Max",
        "Total",
        "Rank",
        "ΔRank",
        "Trn",
        "Bnch",
        "Value",
        gw_w = WIDTH_PTS,
        pts_w = WIDTH_AVAIL,
        avg_w = WIDTH_STAT_SMALL,
        max_w = WIDTH_STAT_SMALL,
        total_w = WIDTH_POINTS,
        rank_w = WIDTH_RANK_WIDE,
        trn_w = WIDTH_STAT_SMALL,
        bnch_w = WIDTH_AVAIL,
        val_w = WIDTH_AVAIL,
    );

    let mut prev_rank: Option<u64> = None;

    for gw in &history.current {
        let rank_str = gw
            .overall_rank
            .map(format_compact_number)
            .unwrap_or_else(|| "-".to_string());

        let rank_change = match (prev_rank, gw.overall_rank) {
            (Some(prev), Some(curr)) => {
                if prev > curr {
                    format!("↑{}", format_compact_number(prev - curr))
                } else if curr > prev {
                    format!("↓{}", format_compact_number(curr - prev))
                } else {
                    "-".to_string()
                }
            }
            _ => "-".to_string(),
        };

        let value = format!("{:.1}", gw.value as f64 / 10.0);

        let (avg_score, max_score) = event_scores.get(&gw.event).cloned().unwrap_or((None, None));
        let avg_str = avg_score
            .map(|s| s.to_string())
            .unwrap_or_else(|| "-".to_string());
        let max_str = max_score
            .map(|s| s.to_string())
            .unwrap_or_else(|| "-".to_string());

        // Color the points based on comparison with average
        let pts_str = match avg_score {
            Some(avg) => color_by_comparison(gw.points, avg as i64),
            _ => format!("{:>width$}", gw.points, width = WIDTH_AVAIL),
        };

        println!(
            "{:<gw_w$} {} {:>avg_w$} {:>max_w$} {:>total_w$} {:>rank_w$} {:>rank_w$} {:>trn_w$} {:>bnch_w$} {:>val_w$}",
            gw.event,
            pts_str,
            avg_str,
            max_str,
            gw.total_points,
            rank_str,
            rank_change,
            gw.event_transfers,
            gw.points_on_bench,
            value,
            gw_w = WIDTH_PTS,
            avg_w = WIDTH_STAT_SMALL,
            max_w = WIDTH_STAT_SMALL,
            total_w = WIDTH_POINTS,
            rank_w = WIDTH_RANK_WIDE,
            trn_w = WIDTH_STAT_SMALL,
            bnch_w = WIDTH_AVAIL,
            val_w = WIDTH_AVAIL,
        );

        prev_rank = gw.overall_rank;
    }

    // 4. Summary Statistics
    println!("\n=== Summary ===\n");

    let total_points = history.current.last().map(|g| g.total_points).unwrap_or(0);
    let final_rank = history
        .current
        .last()
        .and_then(|g| g.overall_rank)
        .map(format_compact_number)
        .unwrap_or_else(|| "N/A".to_string());
    let total_transfers: u64 = history.current.iter().map(|g| g.event_transfers).sum();
    let total_transfer_cost: i64 = history.current.iter().map(|g| g.event_transfers_cost).sum();
    let total_bench_points: i64 = history.current.iter().map(|g| g.points_on_bench).sum();
    let gw_count = history.current.len();
    let avg_points = if gw_count > 0 {
        total_points as f64 / gw_count as f64
    } else {
        0.0
    };

    let best_gw = history.current.iter().max_by_key(|g| g.points);
    let worst_gw = history.current.iter().min_by_key(|g| g.points);

    println!("Total Points:    {}", total_points);
    println!("Overall Rank:    {}", final_rank);
    println!("Avg Points/GW:   {:.1}", avg_points);
    println!("Total Transfers: {}", total_transfers);
    println!("Transfer Costs:  {} pts", total_transfer_cost);
    println!("Bench Points:    {}", total_bench_points);

    if let Some(best) = best_gw {
        println!("Best GW:         {} ({} pts)", best.event, best.points);
    }
    if let Some(worst) = worst_gw {
        println!("Worst GW:        {} ({} pts)", worst.event, worst.points);
    }

    Ok(())
}
