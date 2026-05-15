use deunicode::deunicode;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position};
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};
use crate::utils::{constants::*, formatters::*};

pub async fn handle_availability(
    team: Option<String>,
    name: Option<String>,
    news: Option<String>,
    position: Option<Position>,
    all: bool,
    limit: usize,
) -> Result<()> {
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

            let name_match = if let Some(ref n) = name {
                let normalized_player_name = deunicode(&player.web_name).to_lowercase();
                let normalized_query = deunicode(n).to_lowercase();
                normalized_player_name.contains(&normalized_query)
            } else {
                true
            };

            if !name_match {
                return false;
            }

            let news_match = if let Some(ref n) = news {
                player.news.to_lowercase().contains(&n.to_lowercase())
            } else {
                true
            };

            if !news_match {
                return false;
            }

            let position_match = if let Some(ref pos) = position {
                player.element_type == pos.element_type_id() as u64
            } else {
                true
            };

            if !position_match {
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
        "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:<status_w$}  {:>avail_w$}  {:<news_added_w$}  {:<news_w$}",
        "ID",
        "Name",
        "Pos",
        "Team",
        "Status",
        "Avail",
        "News Added",
        "News",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pos_w = WIDTH_POS,
        team_w = WIDTH_TEAM_SHORT_NAME,
        status_w = WIDTH_STATUS,
        avail_w = WIDTH_AVAIL,
        news_added_w = WIDTH_TIME,
        news_w = WIDTH_NEWS,
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
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:<status_w$}  {:>avail_w$}  {:<news_added_w$}  {:<news_w$}",
            player.id,
            player.web_name,
            Position::from_element_type_id(player.element_type)
                .map(|p| p.display_name().to_string())
                .unwrap_or_default(),
            team_name,
            status_desc,
            avail_display,
            news_added,
            player.news,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            status_w = WIDTH_STATUS,
            avail_w = WIDTH_AVAIL,
            news_added_w = WIDTH_TIME,
            news_w = WIDTH_NEWS,
        );
    }

    Ok(())
}
