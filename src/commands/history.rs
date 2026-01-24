use crate::api::FplClient;
use crate::config::Config;
use clap::Args;

#[derive(Debug, Args)]
pub struct HistoryArgs {
    /// Manager ID (uses configured ID if not provided)
    #[arg(short, long)]
    manager_id: Option<u64>,
}

pub async fn handle_history(args: HistoryArgs) {
    // 1. Determine Manager ID
    let manager_id = if let Some(id) = args.manager_id {
        id
    } else {
        let config = match Config::load() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to load config: {}", e);
                return;
            }
        };

        match config.user.and_then(|u| u.manager_id) {
            Some(id_str) => match id_str.parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Invalid manager_id in config. Please set a numeric ID.");
                    return;
                }
            },
            None => {
                eprintln!(
                    "Manager ID not set. Please run `fplr config set manager-id <ID>` or use --manager-id option."
                );
                return;
            }
        }
    };

    // 2. Fetch Manager History
    let history = match FplClient::fetch_manager_history(manager_id).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to fetch manager history: {}", e);
            return;
        }
    };

    if history.current.is_empty() {
        println!("No history data available for Manager ID: {}", manager_id);
        return;
    }

    // 3. Display Current Season History
    println!("\nManager History (ID: {})", manager_id);
    println!("=== Current Season ===\n");

    println!(
        "{:<3} {:>5} {:>6} {:>10} {:>10} {:>4} {:>5} {:>5}",
        "GW", "Pts", "Total", "Rank", "ΔRank", "Trn", "Bnch", "Value"
    );
    println!("{}", "-".repeat(56));

    let mut prev_rank: Option<u64> = None;

    for gw in &history.current {
        let rank_str = gw
            .overall_rank
            .map(|r| format_number(r))
            .unwrap_or_else(|| "-".to_string());

        let rank_change = match (prev_rank, gw.overall_rank) {
            (Some(prev), Some(curr)) => {
                if prev > curr {
                    format!("↑{}", format_number(prev - curr))
                } else if curr > prev {
                    format!("↓{}", format_number(curr - prev))
                } else {
                    "-".to_string()
                }
            }
            _ => "-".to_string(),
        };

        let value = format!("{:.1}", gw.value as f64 / 10.0);

        println!(
            "{:<3} {:>5} {:>6} {:>10} {:>10} {:>4} {:>5} {:>5}",
            gw.event,
            gw.points,
            gw.total_points,
            rank_str,
            rank_change,
            gw.event_transfers,
            gw.points_on_bench,
            value
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
        .map(|r| format_number(r))
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
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.0}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
