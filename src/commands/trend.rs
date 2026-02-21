use crate::api::FplClient;
use crate::error::Result;
use crate::models::{LiveData, Position};
use crate::utils::formatters::*;
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};
use futures::future::join_all;
use std::collections::HashMap;

pub async fn handle_trend(
    team: Option<String>,
    position: Option<Position>,
    limit: usize,
    weeks: usize,
) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_short_name_map(&bootstrap_data.teams);

    let target_team_ids = if let Some(ref team_name) = team {
        find_team_ids_by_name(&bootstrap_data.teams, team_name)
    } else {
        Vec::new()
    };

    // 1. Filter and sort players by form
    let mut players: Vec<_> = bootstrap_data
        .elements
        .iter()
        .filter(|p| {
            let team_match = if team.is_some() {
                target_team_ids.contains(&p.team)
            } else {
                true
            };
            let pos_match = if let Some(ref pos) = position {
                p.element_type == pos.element_type_id() as u64
            } else {
                true
            };
            team_match && pos_match
        })
        .collect();

    // Sort by form descending
    players.sort_by(|a, b| {
        let a_form: f64 = a.form.parse().unwrap_or(0.0);
        let b_form: f64 = b.form.parse().unwrap_or(0.0);
        b_form.partial_cmp(&a_form).unwrap()
    });

    let top_players: Vec<_> = players.into_iter().take(limit).collect();

    if top_players.is_empty() {
        println!("No players found matching the criteria.");
        return Ok(());
    }

    // 2. Fetch Live Data for recent GWs in parallel
    let current_event = bootstrap_data
        .events
        .iter()
        .filter(|e| e.is_current || e.finished)
        .map(|e| e.id)
        .max()
        .unwrap_or(1) as u32;

    let start_gw = current_event.saturating_sub(weeks as u32 - 1).max(1);
    let gw_range: Vec<u32> = (start_gw..=current_event).collect();

    let live_data_futures: Vec<_> = gw_range
        .iter()
        .map(|&gw| FplClient::fetch_live(gw))
        .collect();
    let live_data_results = join_all(live_data_futures).await;

    let mut live_histories: Vec<LiveData> = Vec::new();
    for res in live_data_results {
        live_histories.push(res?);
    }

    // 3. Map historical points
    let mut player_history_map: HashMap<u64, Vec<i64>> = HashMap::new();
    for live in &live_histories {
        for element in &live.elements {
            player_history_map
                .entry(element.id)
                .or_default()
                .push(element.stats.total_points);
        }
    }

    // Calculate global max for scaling among these top players
    let global_max = top_players
        .iter()
        .filter_map(|p| player_history_map.get(&p.id))
        .flatten()
        .max()
        .copied()
        .unwrap_or(1);

    // 4. Print table
    println!(
        "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>pts_w$}  {:>form_w$}  {:>avail_w$}  {:<trend_w$}",
        "ID",
        "Name",
        "Pos",
        "Team",
        "Cost",
        "Pts",
        "Form",
        "Avail",
        "Trend",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pos_w = WIDTH_POS,
        team_w = WIDTH_TEAM,
        cost_w = WIDTH_COST,
        pts_w = WIDTH_PTS,
        form_w = WIDTH_FORM,
        avail_w = 5,
        trend_w = weeks.max(5)
    );

    for player in top_players {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("???");
        let pos_name = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("???");

        let history = player_history_map
            .get(&player.id)
            .cloned()
            .unwrap_or_default();
        let sparkline = to_sparkline(&history, global_max);
        let cost = format!("{:.1}", player.now_cost as f64 / 10.0);
        let availability =
            format_chance_of_playing(player.chance_of_playing_next_round, &player.news);

        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>pts_w$}  {:>form_w$}  {:>avail_w$}  {:<trend_w$}",
            player.id,
            player.web_name,
            pos_name,
            team_name,
            cost,
            player.total_points,
            player.form,
            availability,
            sparkline,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM,
            cost_w = WIDTH_COST,
            pts_w = WIDTH_PTS,
            form_w = WIDTH_FORM,
            avail_w = 5,
            trend_w = weeks.max(5)
        );
    }

    Ok(())
}
