use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::Position;
use clap::Args;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct TransferArgs {
    /// Show transfers OUT instead of IN
    #[arg(long, conflicts_with = "in_flag")]
    out: bool,

    /// Show transfers IN (default)
    #[arg(long = "in", conflicts_with = "out")]
    in_flag: bool,

    /// Number of players to show
    #[arg(short, long, default_value = "15")]
    limit: usize,

    /// Show all-time transfers instead of this gameweek
    #[arg(long)]
    all_time: bool,
}

pub async fn handle_transfer(args: TransferArgs) -> Result<()> {
    let bootstrap = FplClient::fetch_bootstrap_static().await?;

    // Build team name map
    let team_map: HashMap<u64, String> = bootstrap
        .teams
        .iter()
        .map(|t| (t.id, t.short_name.clone()))
        .collect();

    // Find current gameweek
    let current_gw = bootstrap
        .events
        .iter()
        .find(|e| e.is_current)
        .map(|e| e.id)
        .unwrap_or(1);

    // Sort players by transfer direction (default is IN)
    let mut players = bootstrap.elements;
    let show_out = args.out;

    if show_out {
        if args.all_time {
            players.sort_by(|a, b| b.transfers_out.cmp(&a.transfers_out));
        } else {
            players.sort_by(|a, b| b.transfers_out_event.cmp(&a.transfers_out_event));
        }
    } else {
        if args.all_time {
            players.sort_by(|a, b| b.transfers_in.cmp(&a.transfers_in));
        } else {
            players.sort_by(|a, b| b.transfers_in_event.cmp(&a.transfers_in_event));
        }
    }

    // Print header
    let direction_str = if show_out { "OUT" } else { "IN" };

    let time_str = if args.all_time {
        "All-Time".to_string()
    } else {
        format!("GW{}", current_gw)
    };

    println!("\n=== Top Transfers {} ({}) ===\n", direction_str, time_str);

    println!(
        "{:<4} {:<15} {:<4} {:<4} {:>6} {:>10} {:>10} {:>10}",
        "Rank", "Player", "Team", "Pos", "Price", "IN", "OUT", "Net"
    );

    for (i, player) in players.iter().take(args.limit).enumerate() {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("???");

        let position = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("???");

        let price = format!("£{:.1}", player.now_cost as f64 / 10.0);

        let (transfers_in, transfers_out) = if args.all_time {
            (player.transfers_in, player.transfers_out)
        } else {
            (player.transfers_in_event, player.transfers_out_event)
        };

        let net = transfers_in as i64 - transfers_out as i64;

        let in_str = format_number(transfers_in);
        let out_str = format_number(transfers_out);
        let net_str = if net >= 0 {
            format!("+{}", format_number(net.unsigned_abs()))
        } else {
            format!("-{}", format_number(net.unsigned_abs()))
        };

        // Apply padding first, then color
        let net_padded = format!("{:>10}", net_str);
        let net_colored = if net > 0 {
            net_padded.green().to_string()
        } else if net < 0 {
            net_padded.red().to_string()
        } else {
            net_padded
        };

        println!(
            "{:<4} {:<15} {:<4} {:<4} {:>6} {:>10} {:>10} {}",
            i + 1,
            truncate_name(&player.web_name, 15),
            team_name,
            position,
            price,
            in_str,
            out_str,
            net_colored
        );
    }

    println!();
    Ok(())
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.chars().count() > max_len {
        name.chars().take(max_len - 1).collect::<String>() + "…"
    } else {
        name.to_string()
    }
}
