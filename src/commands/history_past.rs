use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::*;

pub async fn handle_history_past(player_id: u64) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let summary = FplClient::fetch_player_summary(player_id).await?;

    let player_name = bootstrap_data
        .elements
        .iter()
        .find(|e| e.id == player_id)
        .map(|e| e.web_name.as_str())
        .unwrap_or("Unknown Player");

    let past = summary.history_past;

    if past.is_empty() {
        println!(
            "No historical data found for {}: {}",
            player_name, player_id
        );
        return Ok(());
    }

    println!("Historical stats for {}:", player_name);

    // Section 1: Basic & Attacking
    println!("\n[Basic & Attacking]");
    println!(
        "{:<WIDTH_SEASON$} {:>WIDTH_POINTS$} {:>WIDTH_COST_RANGE$} {:>WIDTH_MINUTES$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$}",
        "Season", "Pts", "Cost", "Min", "STR", "G", "A", "CS", "xG", "xA", "xGI"
    );

    for s in &past {
        let cost_range = format!(
            "{:.1}→{:.1}",
            s.start_cost as f64 / 10.0,
            s.end_cost as f64 / 10.0
        );
        println!(
            "{:<WIDTH_SEASON$} {:>WIDTH_POINTS$} {:>WIDTH_COST_RANGE$} {:>WIDTH_MINUTES$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$}",
            s.season_name,
            s.total_points,
            cost_range,
            s.minutes,
            s.starts,
            s.goals_scored,
            s.assists,
            s.clean_sheets,
            s.expected_goals,
            s.expected_assists,
            s.expected_goal_involvements
        );
    }

    // Section 2: Defensive & Discipline
    println!("\n[Defensive & Discipline]");
    println!(
        "{:<WIDTH_SEASON$} {:>WIDTH_STAT_SMALL$} {:>WIDTH_STAT_SMALL$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STAT$}",
        "Season", "S", "GC", "OG", "YC", "RC", "PSV", "PM", "xGC"
    );

    for s in &past {
        println!(
            "{:<WIDTH_SEASON$} {:>WIDTH_STAT_SMALL$} {:>WIDTH_STAT_SMALL$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STR$} {:>WIDTH_STAT$}",
            s.season_name,
            s.saves,
            s.goals_conceded,
            s.own_goals,
            s.yellow_cards,
            s.red_cards,
            s.penalties_saved,
            s.penalties_missed,
            s.expected_goals_conceded
        );
    }

    // Section 3: Bonus & ICT Index
    println!("\n[Bonus & ICT Index]");
    println!(
        "{:<WIDTH_SEASON$} {:>WIDTH_STAT_SMALL$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$}",
        "Season", "B", "BPS", "INF", "CRE", "THR", "ICT"
    );

    for s in &past {
        println!(
            "{:<WIDTH_SEASON$} {:>WIDTH_STAT_SMALL$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$} {:>WIDTH_STAT$}",
            s.season_name, s.bonus, s.bps, s.influence, s.creativity, s.threat, s.ict_index
        );
    }

    Ok(())
}
