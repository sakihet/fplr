use crate::api::FplClient;
use crate::error::Result;
use crate::models::Position;
use crate::models::XaSortBy;
use crate::utils::constants::*;
use crate::utils::formatters::*;
use crate::utils::team_helpers::find_team_ids_by_name;
use std::collections::HashMap;

pub async fn handle_xa(
    sort: XaSortBy,
    team_opt: Option<String>,
    pos_opt: Option<Position>,
    limit: usize,
) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    // Map team id to names
    let team_names: HashMap<u64, String> = data
        .teams
        .iter()
        .map(|t| (t.id, t.short_name.clone()))
        .collect();

    let mut players: Vec<_> = data
        .elements
        .iter()
        .filter(|p| {
            // Team filter
            if let Some(team_name) = &team_opt {
                let target_team_ids = find_team_ids_by_name(&data.teams, team_name);
                if !target_team_ids.contains(&p.team) {
                    return false;
                }
            }

            // Position filter
            if let Some(pos) = &pos_opt
                && p.element_type != pos.element_type_id() as u64
            {
                return false;
            }

            true
        })
        .map(|p| {
            let xa: f64 = p.expected_assists.parse().unwrap_or(0.0);
            let assists = p.assists as f64;
            let diff = assists - xa;
            let ratio = if xa > 0.0 { assists / xa } else { 0.0 };
            let team_name = team_names
                .get(&p.team)
                .cloned()
                .unwrap_or_else(|| "N/A".to_string());
            let pos_name = Position::from_element_type_id(p.element_type)
                .map(|pos| pos.display_name())
                .unwrap_or("N/A");

            (
                p.id,
                p.web_name.clone(),
                pos_name,
                team_name,
                assists,
                xa,
                diff,
                ratio,
            )
        })
        .collect();

    // Sort by selected metric descending
    match sort {
        XaSortBy::Assists => {
            players.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal))
        }
        XaSortBy::Diff => {
            players.sort_by(|a, b| b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal))
        }
        XaSortBy::Ratio => {
            players.sort_by(|a, b| b.7.partial_cmp(&a.7).unwrap_or(std::cmp::Ordering::Equal))
        }
        XaSortBy::Xa => {
            players.sort_by(|a, b| b.5.partial_cmp(&a.5).unwrap_or(std::cmp::Ordering::Equal))
        }
    }

    println!(
        "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>ast_w$}  {:>xa_w$}  {:>diff_w$}  {:>ratio_w$}",
        "ID",
        "Player",
        "Pos",
        "Team",
        "A",
        "xA",
        "Diff",
        "Ratio",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pos_w = WIDTH_POS,
        team_w = WIDTH_TEAM_SHORT_NAME,
        ast_w = 4,
        xa_w = 6,
        diff_w = 6,
        ratio_w = 6,
    );

    for (id, name, pos, team, assists, xa, diff, ratio) in players.into_iter().take(limit) {
        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>ast_w$.0}  {:>xa_w$.2}  {:>diff_w$.2}  {:>ratio_w$.2}",
            id,
            truncate(&name, WIDTH_NAME),
            pos,
            team,
            assists,
            xa,
            diff,
            ratio,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            ast_w = 4,
            xa_w = 6,
            diff_w = 6,
            ratio_w = 6,
        );
    }

    Ok(())
}
