use crate::api::FplClient;
use crate::error::Result;
use crate::utils::event_helpers::get_effective_event_id;
use crate::utils::formatters::*;
use crate::utils::player_helpers::create_player_map;

pub async fn handle_dream_team(gw: Option<u32>) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let player_map = create_player_map(&bootstrap_data.elements);

    let event_id = match get_effective_event_id(&bootstrap_data.events, gw) {
        Some(id) => id,
        None => {
            println!("Could not determine current Gameweek.");
            return Ok(());
        }
    };

    let data = FplClient::fetch_dream_team(event_id).await?;
    let mut team = data.team;
    team.sort_by(|a, b| b.points.cmp(&a.points));

    println!(
        "{:>id_w$}  {:<name_w$}  {:>pts_w$}",
        "ID",
        "Name",
        "Pts",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pts_w = WIDTH_PTS,
    );
    for t in team.iter() {
        let name = player_map
            .get(&t.element)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        println!(
            "{:>id_w$}  {:<name_w$}  {:>pts_w$}",
            t.element,
            name,
            t.points,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pts_w = WIDTH_PTS,
        );
    }

    Ok(())
}
