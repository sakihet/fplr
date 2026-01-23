use crate::api::FplClient;
use crate::config::Config;
use crate::models::{Pick, Position};
use crate::utils::team_helpers::create_team_map;
use clap::Args;
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct MyTeamArgs {
    /// Optional Gameweek ID. If not provided, uses the current Gameweek.
    #[arg(short, long)]
    gw: Option<u32>,
}

pub async fn handle_my_team(args: MyTeamArgs) {
    // 1. Load Config
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };

    let manager_id = match config.user.and_then(|u| u.manager_id) {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Invalid manager_id in config. Please set a numeric ID.");
                return;
            }
        },
        None => {
            eprintln!("Manager ID not set. Please run `fplr config set manager-id <ID>` first.");
            return;
        }
    };

    // 2. Fetch Bootstrap Static to get current GW and player details
    let bootstrap = match FplClient::fetch_bootstrap_static().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to fetch bootstrap data: {}", e);
            return;
        }
    };

    // Determine Gameweek
    let event_id = if let Some(gw) = args.gw {
        gw
    } else {
        match bootstrap.events.iter().find(|e| e.is_current) {
            Some(e) => e.id as u32,
            None => {
                // Fallback: if no current event, maybe look for next one and pick previous?
                // Or simply pick the last one that finished?
                // For now, let's try to find the one with is_next=true and subtract 1, or just error.
                if let Some(next) = bootstrap.events.iter().find(|e| e.is_next) {
                    if next.id > 1 {
                        (next.id - 1) as u32
                    } else {
                        eprintln!("Season hasn't started yet.");
                        return;
                    }
                } else {
                    // Maybe season finished? Get the last event
                    if let Some(last) = bootstrap.events.last() {
                        last.id as u32
                    } else {
                        eprintln!("Could not determine current Gameweek.");
                        return;
                    }
                }
            }
        }
    };

    println!(
        "Fetching team for Manager ID: {} (GW {})",
        manager_id, event_id
    );

    // 3. Fetch Manager Picks
    let picks_data = match FplClient::fetch_manager_picks(manager_id, event_id).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to fetch manager picks: {}", e);
            return;
        }
    };

    // 4. Fetch Live Data for points
    let live_data = match FplClient::fetch_live(event_id).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to fetch live data: {}", e);
            // We can continue without live points if needed, but for now let's return
            return;
        }
    };

    // Helper maps
    let team_map = create_team_map(&bootstrap.teams);
    let player_map: HashMap<u64, &crate::models::Element> =
        bootstrap.elements.iter().map(|p| (p.id, p)).collect();

    let live_map: HashMap<u64, &crate::models::LiveStats> = live_data
        .elements
        .iter()
        .map(|e| (e.id, &e.stats))
        .collect();

    // 5. Display
    // Sort picks by position: GKP(1), DEF(2), MID(3), FWD(4), then Sub
    // Picks 1-11 are starters, 12-15 are bench.

    let mut starters: Vec<&Pick> = picks_data
        .picks
        .iter()
        .filter(|p| p.position <= 11)
        .collect();
    let bench: Vec<&Pick> = picks_data
        .picks
        .iter()
        .filter(|p| p.position > 11)
        .collect();

    // Sort starters by position type
    starters.sort_by_key(|p| {
        if let Some(player) = player_map.get(&p.element) {
            player.element_type
        } else {
            99 // unknown
        }
    });

    // Header
    println!(
        "\n{:<15} GW{} Points: {}",
        "My Team", event_id, picks_data.entry_history.points
    );
    println!(
        "Overall Rank: {}",
        picks_data
            .entry_history
            .overall_rank
            .map(|r| r.to_string())
            .unwrap_or("N/A".to_string())
    );
    println!("");
    println!(
        "{:<4} {:<4} {:<20} {:<15} {:<5} {:<6} {:<6} {:<32}",
        "ID", "Pos", "Name", "Team", "Pts", "Cost", "Form", "Status"
    );

    // Function to print a player row
    let print_player = |pick: &Pick, _is_bench: bool| {
        let player_opt = player_map.get(&pick.element);
        let live_opt = live_map.get(&pick.element);

        if let Some(player) = player_opt {
            let pos_name = Position::from_element_type_id(player.element_type)
                .map(|p| p.display_name())
                .unwrap_or("???");

            let team_name = team_map
                .get(&player.team)
                .map(|s| s.as_str())
                .unwrap_or("???");

            let mut name_display = player.web_name.clone();
            if pick.is_captain {
                name_display.push_str(" (C)");
            }
            if pick.is_vice_captain {
                name_display.push_str(" (VC)");
            }

            let points = live_opt.map(|l| l.total_points).unwrap_or(0);
            // Apply multiplier for display (e.g. Captain points doubled)
            let final_points = points * (pick.multiplier as i64);

            let cost = format!("{:.1}", player.now_cost as f64 / 10.0);

            println!(
                "{:<4} {:<4} {:<20} {:<15} {:<5} {:<6} {:<6} {:<32}",
                player.id,
                pos_name,
                name_display,
                team_name,
                final_points,
                cost,
                player.form,
                player.news.chars().take(32).collect::<String>()
            );
        } else {
            println!("Unknown Player ID: {}", pick.element);
        }
    };

    println!("Starters:");
    for pick in starters {
        print_player(pick, false);
    }

    println!("\nBench:");
    for pick in bench {
        print_player(pick, true);
    }
}
