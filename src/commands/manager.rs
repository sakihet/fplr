use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::event_helpers::get_effective_event_id;
use crate::utils::player_helpers::create_player_map;
use crate::utils::team_helpers::create_team_short_name_map;

pub async fn handle_manager(manager_id: u64, gw: Option<u32>) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;

    let event_id =
        get_effective_event_id(&bootstrap_data.events, gw).ok_or(FplrError::NoNextEvent)?;

    let player_map = create_player_map(&bootstrap_data.elements);
    let team_map = create_team_short_name_map(&bootstrap_data.teams);
    let player_team_id_map: HashMap<u64, u64> = bootstrap_data
        .elements
        .iter()
        .map(|e| (e.id, e.team))
        .collect();

    let live_data = FplClient::fetch_live(event_id).await?;
    let points_map: HashMap<u64, i64> = live_data
        .elements
        .iter()
        .map(|element| (element.id, element.stats.total_points))
        .collect();

    let picks = FplClient::fetch_manager_picks(manager_id, event_id).await?;

    println!("Manager ID: {} (GW {})", manager_id, event_id);
    println!("\n--- Starting XI ---");
    println!(
        "{:<4} {:<25} {:<6} {:<6} {:<4} {:<4} {:<4}",
        "ID", "Name", "Team", "Pos", "C", "VC", "Pts"
    );

    for (i, pick) in picks.picks.iter().enumerate() {
        if i == 11 {
            println!("\n--- Bench ---");
            println!(
                "{:<4} {:<25} {:<6} {:<6} {:<4} {:<4} {:<4}",
                "ID", "Name", "Team", "Pos", "C", "VC", "Pts"
            );
        }

        let name = player_map
            .get(&pick.element)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let team_short = player_team_id_map
            .get(&pick.element)
            .and_then(|tid| team_map.get(tid))
            .map(|s| s.as_str())
            .unwrap_or("???");

        let points = points_map.get(&pick.element).copied().unwrap_or(0);

        println!(
            "{:<4} {:<25} {:<6} {:<6} {:<4} {:<4} {:<4}",
            pick.element,
            name,
            team_short,
            format_position(pick.position),
            if pick.is_captain { "Y" } else { "" },
            if pick.is_vice_captain { "Y" } else { "" },
            points,
        );
    }

    Ok(())
}

fn format_position(pos: u32) -> &'static str {
    match pos {
        1 => "GK",
        2..=5 => "DEF",
        6..=10 => "MID",
        11..=15 => "FWD",
        _ => "SUB",
    }
}
