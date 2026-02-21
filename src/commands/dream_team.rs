use crate::api::FplClient;
use crate::error::Result;
use crate::utils::formatters::*;
use crate::utils::player_helpers::create_player_map;

pub async fn handle_dream_team(event_id: u32) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let player_map = create_player_map(&bootstrap_data.elements);

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
