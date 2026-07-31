use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::{WIDTH_FULL_NAME, WIDTH_ID, WIDTH_STAT_SMALL};
use crate::utils::player_helpers::create_player_map;

pub async fn handle_pick(manager_id: u64, event_id: u32) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let player_map = create_player_map(&bootstrap_data.elements);

    let live_data = FplClient::fetch_live(event_id).await?;
    let points_map: HashMap<u64, i64> = live_data
        .elements
        .iter()
        .map(|element| (element.id, element.stats.total_points))
        .collect();

    let picks = FplClient::fetch_manager_picks(manager_id, event_id).await?;

    println!(
        "{:<id_w$} {:<name_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$}",
        "ID",
        "Name",
        "Pos",
        "C",
        "VC",
        "Pts",
        id_w = WIDTH_ID,
        name_w = WIDTH_FULL_NAME,
        stat_w = WIDTH_STAT_SMALL,
    );
    for pick in picks.picks.iter() {
        let name = player_map
            .get(&pick.element)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let points = points_map.get(&pick.element).copied().unwrap_or(0);

        println!(
            "{:<id_w$} {:<name_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$} {:<stat_w$}",
            pick.element,
            name,
            pick.position,
            if pick.is_captain { "Y" } else { "N" },
            if pick.is_vice_captain { "Y" } else { "N" },
            points,
            id_w = WIDTH_ID,
            name_w = WIDTH_FULL_NAME,
            stat_w = WIDTH_STAT_SMALL,
        );
    }

    Ok(())
}
