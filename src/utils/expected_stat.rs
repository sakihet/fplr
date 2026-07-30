use crate::api::FplClient;
use crate::error::Result;
use crate::models::Element;
use crate::models::Position;
use crate::utils::constants::*;
use crate::utils::formatters::*;
use crate::utils::team_helpers::find_team_ids_by_name;
use std::collections::HashMap;

/// Which column to sort the resulting table by.
pub enum StatSort {
    Actual,
    Expected,
    Diff,
    Ratio,
}

/// Describes an "actual vs expected" stat table (xA, xG, xGC, xGI).
pub struct ExpectedStatSpec {
    pub actual_label: &'static str,
    pub expected_label: &'static str,
    pub actual_width: usize,
    pub actual_fn: fn(&Element) -> f64,
    pub expected_fn: fn(&Element) -> f64,
}

pub async fn print_expected_stat_table(
    sort: StatSort,
    team_opt: Option<String>,
    pos_opt: Option<Position>,
    limit: usize,
    spec: ExpectedStatSpec,
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
            let actual = (spec.actual_fn)(p);
            let expected = (spec.expected_fn)(p);
            let diff = actual - expected;
            let ratio = if expected > 0.0 {
                actual / expected
            } else {
                0.0
            };
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
                actual,
                expected,
                diff,
                ratio,
            )
        })
        .collect();

    // Sort by selected metric descending
    match sort {
        StatSort::Actual => {
            players.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal))
        }
        StatSort::Expected => {
            players.sort_by(|a, b| b.5.partial_cmp(&a.5).unwrap_or(std::cmp::Ordering::Equal))
        }
        StatSort::Diff => {
            players.sort_by(|a, b| b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal))
        }
        StatSort::Ratio => {
            players.sort_by(|a, b| b.7.partial_cmp(&a.7).unwrap_or(std::cmp::Ordering::Equal))
        }
    }

    println!(
        "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>act_w$}  {:>exp_w$}  {:>diff_w$}  {:>ratio_w$}",
        "ID",
        "Player",
        "Pos",
        "Team",
        spec.actual_label,
        spec.expected_label,
        "Diff",
        "Ratio",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pos_w = WIDTH_POS,
        team_w = WIDTH_TEAM_SHORT_NAME,
        act_w = spec.actual_width,
        exp_w = WIDTH_STAT,
        diff_w = WIDTH_STAT,
        ratio_w = WIDTH_STAT,
    );

    for (id, name, pos, team, actual, expected, diff, ratio) in players.into_iter().take(limit) {
        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>act_w$.0}  {:>exp_w$.2}  {:>diff_w$.2}  {:>ratio_w$.2}",
            id,
            truncate(&name, WIDTH_NAME),
            pos,
            team,
            actual,
            expected,
            diff,
            ratio,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            act_w = spec.actual_width,
            exp_w = WIDTH_STAT,
            diff_w = WIDTH_STAT,
            ratio_w = WIDTH_STAT,
        );
    }

    Ok(())
}
