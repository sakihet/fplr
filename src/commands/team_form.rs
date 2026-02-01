use crate::api::FplClient;
use crate::error::Result;
use std::collections::HashMap;

pub async fn handle_team_form() -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let mut team_total_form: HashMap<u64, f64> = HashMap::new();
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
            let player_count = team_player_count.get(&team.id).cloned().unwrap_or(0);
            let top_player =
                team_top_player
                    .get(&team.id)
                    .cloned()
                    .unwrap_or((0, "N/A".to_string(), 0.0));
            (team.name.clone(), total_form, player_count, top_player)
        })
        .collect();

    // Sort by total form descending
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!(
        "{:<20} {:<12} {:<10} {:<20} {:<8} {:<8}",
        "Team", "Total Form", "Players", "Top Player", "Form", "ID"
    );

    for (name, total, count, top) in results {
        println!(
            "{:<20} {:<12.1} {:<10} {:<20} {:<8.1} {:<8}",
            name, total, count, top.1, top.2, top.0
        );
    }

    Ok(())
}
