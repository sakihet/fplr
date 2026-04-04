use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::formatters::*;
use crate::utils::team_helpers::find_team_ids_by_name;
use std::collections::HashMap;

pub async fn handle_talisman(team_opt: Option<String>) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let mut team_total_points: HashMap<u64, u64> = HashMap::new();
    let mut team_players: HashMap<u64, Vec<(u64, String, String, u64)>> = HashMap::new();

    // Map team id to team names
    let mut team_names: HashMap<u64, String> = HashMap::new();
    let mut team_short_names: HashMap<u64, String> = HashMap::new();
    for team in &data.teams {
        team_names.insert(team.id, team.name.clone());
        team_short_names.insert(team.id, team.short_name.clone());
    }

    // Calculate total points per team and collect players
    for player in &data.elements {
        let pts = player.total_points as u64;
        *team_total_points.entry(player.team).or_insert(0) += pts;

        let pos_name = crate::models::Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("N/A")
            .to_string();

        team_players.entry(player.team).or_default().push((
            player.id,
            player.web_name.clone(),
            pos_name,
            pts,
        ));
    }

    if let Some(team_name) = team_opt {
        // Output for a specific team
        let target_team_ids = find_team_ids_by_name(&data.teams, &team_name);
        if target_team_ids.is_empty() {
            println!("No team found matching '{}'", team_name);
            return Ok(());
        }

        let team_id = target_team_ids[0];
        let t_name = team_names.get(&team_id).unwrap();
        let total_pts = team_total_points.get(&team_id).copied().unwrap_or(0);

        let mut players = team_players.remove(&team_id).unwrap_or_default();

        // Sort players by points descending
        players.sort_by(|a, b| b.3.cmp(&a.3));

        println!("Team: {} (Total Points: {})", t_name, total_pts);
        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:>pts_w$}  {:>pct_w$}",
            "ID",
            "Player",
            "Pos",
            "Pts",
            "% Cont.",
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            pts_w = 4,
            pct_w = 7,
        );

        for (id, p_name, pos, pts) in players {
            let pct = if total_pts > 0 {
                (pts as f64 / total_pts as f64) * 100.0
            } else {
                0.0
            };

            println!(
                "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:>pts_w$}  {:>pct_w$.1}%",
                id,
                truncate(&p_name, WIDTH_NAME),
                pos,
                pts,
                pct,
                id_w = WIDTH_ID,
                name_w = WIDTH_NAME,
                pos_w = WIDTH_POS,
                pts_w = 4,
                pct_w = 6, // 6 to leave room for the % sign
            );
        }
    } else {
        // Output top player for each team
        let mut results = Vec::new();

        for (team_id, mut players) in team_players {
            let t_name = team_names.get(&team_id).unwrap().clone();
            let t_short_name = team_short_names.get(&team_id).unwrap().clone();
            let total_pts = team_total_points.get(&team_id).copied().unwrap_or(0);

            if !players.is_empty() {
                // Find top player
                players.sort_by(|a, b| b.3.cmp(&a.3));
                let (top_id, top_name, _pos, top_pts) = players.remove(0);

                let pct = if total_pts > 0 {
                    (top_pts as f64 / total_pts as f64) * 100.0
                } else {
                    0.0
                };

                results.push((
                    t_short_name,
                    t_name,
                    top_id,
                    top_name,
                    top_pts,
                    total_pts,
                    pct,
                ));
            }
        }

        // Sort by percentage descending
        results.sort_by(|a, b| b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal));

        println!(
            "{:<team_w$}  {:<name_w$}  {:<top_w$}  {:>id_w$}  {:>pts_w$}  {:>tpts_w$}  {:>pct_w$}",
            "Team",
            "Name",
            "Top Player",
            "ID",
            "Pts",
            "Team Pts",
            "% Cont.",
            team_w = WIDTH_TEAM_SHORT_NAME,
            name_w = WIDTH_TEAM_NAME,
            top_w = WIDTH_NAME,
            id_w = WIDTH_ID,
            pts_w = 4,
            tpts_w = 8,
            pct_w = 7,
        );

        for (t_short_name, t_name, p_id, p_name, pts, total_pts, pct) in results {
            println!(
                "{:<team_w$}  {:<name_w$}  {:<top_w$}  {:>id_w$}  {:>pts_w$}  {:>tpts_w$}  {:>pct_w$.1}%",
                t_short_name,
                truncate(&t_name, WIDTH_TEAM_NAME),
                truncate(&p_name, WIDTH_NAME),
                p_id,
                pts,
                total_pts,
                pct,
                team_w = WIDTH_TEAM_SHORT_NAME,
                name_w = WIDTH_TEAM_NAME,
                top_w = WIDTH_NAME,
                id_w = WIDTH_ID,
                pts_w = 4,
                tpts_w = 8,
                pct_w = 6,
            );
        }
    }

    Ok(())
}
