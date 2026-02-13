use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position};
use crate::utils::formatters::format_datetime_local;
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};
use owo_colors::OwoColorize;

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
        "{:<4} {:<20} {:<6} {:<4} {:<14} {:<8} {:<24} {:<30}",
        "ID", "Name", "Team", "Pos", "Status", "Chance", "News Added", "News"
    );

    for player in players.iter().take(limit) {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let status_desc = player.status.display_name();

        let chance_val = player.chance_of_playing_next_round;
        let chance_str = chance_val
            .map(|c| format!("{}%", c))
            .unwrap_or_else(|| "N/A".to_string());

        let chance_padding = " ".repeat(8usize.saturating_sub(chance_str.len()));

        let colored_chance = match chance_val {
            Some(75) => chance_str.yellow().to_string(),
            Some(50) => chance_str.truecolor(255, 165, 0).to_string(),
            Some(0) => chance_str.red().to_string(),
            _ => chance_str,
        };

        let news_added = player
            .news_added
            .as_ref()
            .map(|s| format_datetime_local(s))
            .unwrap_or_else(|| "N/A".to_string());

        println!(
            "{:<4} {:<20} {:<6} {:<4} {:<14} {}{} {:<24} {:<30}",
            player.id,
            player.web_name,
            team_name,
            Position::from_element_type_id(player.element_type)
                .map(|p| p.display_name().to_string())
                .unwrap_or_default(),
            status_desc,
            colored_chance,
            chance_padding,
            news_added,
            player.news
        );
    }

    Ok(())
}
