use deunicode::deunicode;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position, SortBy};
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};

#[derive(Debug, Default)]
pub struct PlayerFilterArgs {
    pub sort: SortBy,
    pub position: Option<Position>,
    pub limit: usize,
    pub team: Option<String>,
    pub name: Option<String>,
    pub min_cost: Option<f64>,
    pub max_cost: Option<f64>,
    pub available: bool,
}

pub async fn handle_player(args: PlayerFilterArgs) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let team_map = create_team_short_name_map(&data.teams);
    let target_team_ids = if let Some(ref team_name) = args.team {
        find_team_ids_by_name(&data.teams, team_name)
    } else {
        Vec::new()
    };

    let mut players: Vec<Element> = data
        .elements
        .into_iter()
        .filter(|player| {
            let position_match = if let Some(ref pos) = args.position {
                player.element_type == pos.element_type_id() as u64
            } else {
                true
            };
            let team_match = if args.team.is_some() {
                target_team_ids.contains(&player.team)
            } else {
                true
            };
            let name_match = if let Some(ref n) = args.name {
                let normalized_player_name = deunicode(&player.web_name).to_lowercase();
                let normalized_query = deunicode(n).to_lowercase();
                normalized_player_name.contains(&normalized_query)
            } else {
                true
            };
            let cost_match = {
                let p_cost = player.now_cost as f64;
                let min_match = if let Some(min) = args.min_cost {
                    p_cost >= min * 10.0
                } else {
                    true
                };
                let max_match = if let Some(max) = args.max_cost {
                    p_cost <= max * 10.0
                } else {
                    true
                };
                min_match && max_match
            };
            let available_match = if args.available {
                player
                    .status
                    .is_available(player.chance_of_playing_next_round)
            } else {
                true
            };
            position_match && team_match && name_match && cost_match && available_match
        })
        .collect();

    match args.sort {
        SortBy::Cost => players.sort_by(|a, b| b.now_cost.cmp(&a.now_cost)),
        SortBy::Form => players.sort_by(|a, b| {
            let form_a = a.form.parse::<f64>().unwrap_or(0.0);
            let form_b = b.form.parse::<f64>().unwrap_or(0.0);
            form_b.partial_cmp(&form_a).unwrap()
        }),
        SortBy::Points => players.sort_by(|a, b| b.total_points.cmp(&a.total_points)),
        SortBy::SelectedBy => players.sort_by(|a, b| {
            let selected_by_a = a.selected_by_percent.parse::<f64>().unwrap_or(0.0);
            let selected_by_b = b.selected_by_percent.parse::<f64>().unwrap_or(0.0);
            selected_by_b.partial_cmp(&selected_by_a).unwrap()
        }),
    }

    println!(
        "{:<4} {:<20} {:<4} {:<6} {:<6} {:<8} {:<6} {:<8} {:<30}",
        "ID", "Name", "Pos", "Team", "Cost", "Selected", "Form", "Points", "News"
    );

    for player in players.iter().take(args.limit) {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        println!(
            "{:<4} {:<20} {:<4} {:<6} {:<6} {:<8} {:<6} {:<8} {:<30}",
            player.id,
            player.web_name,
            Position::from_element_type_id(player.element_type)
                .map(|p| p.display_name().to_string())
                .unwrap_or("N/A".to_string()),
            team_name,
            format!("{:.1}", player.now_cost as f64 / 10.0),
            player.selected_by_percent,
            player.form,
            player.total_points,
            player.news,
        );
    }

    Ok(())
}
