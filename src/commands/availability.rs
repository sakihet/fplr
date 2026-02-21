use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position};
use crate::utils::formatters::*;
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};

pub async fn handle_availability(team: Option<String>, all: bool, limit: usize) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let team_map = create_team_short_name_map(&data.teams);
    let target_team_ids = if let Some(ref team_name) = team {
        find_team_ids_by_name(&data.teams, team_name)
    } else {
        Vec::new()
    };

    let mut players: Vec<Element> = data
        .elements
        .into_iter()
        .filter(|player| {
            let team_match = if team.is_some() {
                target_team_ids.contains(&player.team)
            } else {
                true
            };

            if !team_match {
                return false;
            }

            // If not "all", only show players with news or non-available status
            if !all
                && player
                    .status
                    .is_available(player.chance_of_playing_next_round)
                && player.news.is_empty()
            {
                return false;
            }

            true
        })
        .collect();

    if players.is_empty() {
        println!("No availability issues found.");
        return Ok(());
    }

    // Sort players by news_added (latest first)
    players.sort_by(|a, b| b.news_added.cmp(&a.news_added));

    println!(
        "{:>id_w$}  {:<name_w$}  {:<team_w$}  {:<pos_w$}  {:<status_w$}  {:>avail_w$}  {:<news_added_w$}  {:<news_w$}",
        "ID",
        "Name",
        "Team",
        "Pos",
        "Status",
        "Avail",
        "News Added",
        "News",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        team_w = WIDTH_TEAM,
        pos_w = WIDTH_POS,
        status_w = 14,
        avail_w = 5,
        news_added_w = 24,
        news_w = 30,
    );

    for player in players.iter().take(limit) {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let status_desc = player.status.display_name();

        let avail_display =
            format_chance_of_playing(player.chance_of_playing_next_round, &player.news);

        let news_added = player
            .news_added
            .as_ref()
            .map(|s| format_datetime_local(s))
            .unwrap_or_else(|| "N/A".to_string());

        println!(
            "{:>id_w$}  {:<name_w$}  {:<team_w$}  {:<pos_w$}  {:<status_w$}  {:>avail_w$}  {:<news_added_w$}  {:<news_w$}",
            player.id,
            player.web_name,
            team_name,
            Position::from_element_type_id(player.element_type)
                .map(|p| p.display_name().to_string())
                .unwrap_or_default(),
            status_desc,
            avail_display,
            news_added,
            player.news,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            team_w = WIDTH_TEAM,
            pos_w = WIDTH_POS,
            status_w = 14,
            avail_w = 5,
            news_added_w = 24,
            news_w = 30,
        );
    }

    Ok(())
}
