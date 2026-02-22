use crate::api::FplClient;
use crate::error::Result;
use crate::models::TeamFormSortBy;
use crate::utils::constants::*;
use crate::utils::formatters::*;
use std::collections::HashMap;

pub async fn handle_team_form(sort_by: &TeamFormSortBy) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let mut team_total_form: HashMap<u64, f64> = HashMap::new();
    let mut team_pos_form: HashMap<u64, HashMap<u64, f64>> = HashMap::new();
    let mut team_player_count: HashMap<u64, usize> = HashMap::new();
    let mut team_top_player: HashMap<u64, (u64, String, f64)> = HashMap::new();

    for player in data.elements.iter() {
        // Availability check (same logic as player command)
        let is_available = player
            .status
            .is_available(player.chance_of_playing_next_round);
        if !is_available {
            continue;
        }

        let form: f64 = player.form.parse().unwrap_or(0.0);

        *team_total_form.entry(player.team).or_insert(0.0) += form;

        let pos_forms = team_pos_form.entry(player.team).or_default();
        *pos_forms.entry(player.element_type).or_insert(0.0) += form;

        *team_player_count.entry(player.team).or_insert(0) += 1;

        let current_top = team_top_player
            .entry(player.team)
            .or_insert((0, String::new(), -1.0));
        if form > current_top.2 {
            *current_top = (player.id, player.web_name.clone(), form);
        }
    }

    let mut results: Vec<_> = data
        .teams
        .iter()
        .map(|team| {
            let total_form = team_total_form.get(&team.id).cloned().unwrap_or(0.0);
            let pos_forms = team_pos_form.get(&team.id).cloned().unwrap_or_default();

            let fwd_form = pos_forms.get(&4).cloned().unwrap_or(0.0);
            let mid_form = pos_forms.get(&3).cloned().unwrap_or(0.0);
            let def_form = pos_forms.get(&2).cloned().unwrap_or(0.0);
            let gkp_form = pos_forms.get(&1).cloned().unwrap_or(0.0);

            let player_count = team_player_count.get(&team.id).cloned().unwrap_or(0);
            let top_player =
                team_top_player
                    .get(&team.id)
                    .cloned()
                    .unwrap_or((0, "N/A".to_string(), 0.0));
            (
                team.name.clone(),
                total_form,
                fwd_form,
                mid_form,
                def_form,
                gkp_form,
                player_count,
                top_player,
            )
        })
        .collect();

    // Sort
    match sort_by {
        TeamFormSortBy::Total => {
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
        }
        TeamFormSortBy::Forward => {
            results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal))
        }
        TeamFormSortBy::Midfielder => {
            results.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal))
        }
        TeamFormSortBy::Defender => {
            results.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal))
        }
        TeamFormSortBy::Goalkeeper => {
            results.sort_by(|a, b| b.5.partial_cmp(&a.5).unwrap_or(std::cmp::Ordering::Equal))
        }
    }

    println!(
        "{:<name_w$}  {:>form_w$}  {:>pos_w$}  {:>pos_w$}  {:>pos_w$}  {:>pos_w$}  {:>count_w$}  {:<top_w$}  {:>form_w$}  {:>id_w$}",
        "Team",
        "Form",
        "FWD",
        "MID",
        "DEF",
        "GKP",
        "Players",
        "Top Player",
        "Form",
        "ID",
        name_w = WIDTH_TEAM_NAME,
        form_w = WIDTH_FORM,
        pos_w = 4,
        count_w = 7,
        top_w = WIDTH_NAME,
        id_w = WIDTH_ID
    );

    for (name, total, fwd_form, mid_form, def_form, gkp_form, count, top) in results {
        println!(
            "{:<name_w$}  {:>form_w$.1}  {:>pos_w$.1}  {:>pos_w$.1}  {:>pos_w$.1}  {:>pos_w$.1}  {:>count_w$}  {:<top_w$}  {:>form_w$.1}  {:>id_w$}",
            truncate(&name, WIDTH_TEAM_NAME),
            total,
            fwd_form,
            mid_form,
            def_form,
            gkp_form,
            count,
            truncate(&top.1, WIDTH_NAME),
            top.2,
            top.0,
            name_w = WIDTH_TEAM_NAME,
            form_w = WIDTH_FORM,
            pos_w = 4,
            count_w = 7,
            top_w = WIDTH_NAME,
            id_w = WIDTH_ID
        );
    }

    Ok(())
}
