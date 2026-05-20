use crate::api::FplClient;
use crate::error::Result;

pub async fn handle_history_past(player_id: u64) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let player_name = bootstrap_data
        .elements
        .iter()
        .find(|e| e.id == player_id)
        .map(|e| e.web_name.as_str())
        .unwrap_or("Unknown Player");

    let summary = FplClient::fetch_player_summary(player_id).await?;
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
        "{:<10} {:>5} {:>5} {:>5} {:>3} {:>3} {:>3} {:>3} {:>6} {:>6}",
        "Season", "Pts", "Cost", "Min", "STR", "G", "A", "CS", "xG", "xA"
    );

    for s in &past {
        println!(
            "{:<10} {:>5} {:>5.1} {:>5} {:>3} {:>3} {:>3} {:>3} {:>6} {:>6}",
            s.season_name,
            s.total_points,
            s.end_cost as f64 / 10.0,
            s.minutes,
            s.starts,
            s.goals_scored,
            s.assists,
            s.clean_sheets,
            s.expected_goals,
            s.expected_assists
        );
    }

    // Section 2: Defensive & Discipline
    println!("\n[Defensive & Discipline]");
    println!(
        "{:<10} {:>4} {:>4} {:>3} {:>3} {:>3}",
        "Season", "S", "GC", "OG", "YC", "RC"
    );

    for s in &past {
        println!(
            "{:<10} {:>4} {:>4} {:>3} {:>3} {:>3}",
            s.season_name, s.saves, s.goals_conceded, s.own_goals, s.yellow_cards, s.red_cards
        );
    }

    // Section 3: Bonus & ICT Index
    println!("\n[Bonus & ICT Index]");
    println!(
        "{:<10} {:>4} {:>5} {:>6} {:>6} {:>6} {:>6}",
        "Season", "B", "BPS", "INF", "CRE", "THR", "ICT"
    );

    for s in &past {
        println!(
            "{:<10} {:>4} {:>5} {:>6} {:>6} {:>6} {:>6}",
            s.season_name, s.bonus, s.bps, s.influence, s.creativity, s.threat, s.ict_index
        );
    }

    Ok(())
}
