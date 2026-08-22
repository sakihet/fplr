use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::models::Position;
use crate::utils::constants::{WIDTH_ABBR, WIDTH_ID, WIDTH_LONG_NAME, WIDTH_STAT_SMALL};
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
    let player_pos_map: HashMap<u64, u64> = bootstrap_data
        .elements
        .iter()
        .map(|e| (e.id, e.element_type))
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
        "{:<id_w$} {:<name_w$} {:<abbr_w$} {:<abbr_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$}",
        "ID",
        "Name",
        "Team",
        "Pos",
        "C",
        "VC",
        "Pts",
        id_w = WIDTH_ID,
        name_w = WIDTH_LONG_NAME,
        abbr_w = WIDTH_ABBR,
        stat_w = WIDTH_STAT_SMALL,
    );

    for (i, pick) in picks.picks.iter().enumerate() {
        if i == 11 {
            println!("\n--- Bench ---");
            println!(
                "{:<id_w$} {:<name_w$} {:<abbr_w$} {:<abbr_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$}",
                "ID",
                "Name",
                "Team",
                "Pos",
                "C",
                "VC",
                "Pts",
                id_w = WIDTH_ID,
                name_w = WIDTH_LONG_NAME,
                abbr_w = WIDTH_ABBR,
                stat_w = WIDTH_STAT_SMALL,
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

        let pos_str = player_pos_map
            .get(&pick.element)
            .and_then(|&etid| Position::from_element_type_id(etid))
            .map(|p| p.display_name())
            .unwrap_or("???");

        let points = points_map.get(&pick.element).copied().unwrap_or(0);

        println!(
            "{:<id_w$} {:<name_w$} {:<abbr_w$} {:<abbr_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$}",
            pick.element,
            name,
            team_short,
            pos_str,
            if pick.is_captain { "Y" } else { "" },
            if pick.is_vice_captain { "Y" } else { "" },
            points,
            id_w = WIDTH_ID,
            name_w = WIDTH_LONG_NAME,
            abbr_w = WIDTH_ABBR,
            stat_w = WIDTH_STAT_SMALL,
        );
    }

    Ok(())
}
