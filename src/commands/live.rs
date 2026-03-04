use crate::api::FplClient;
use crate::error::Result;
use crate::models::StatsPoints;
use crate::utils::constants::*;
use crate::utils::event_helpers::get_effective_event_id;
use crate::utils::player_helpers::create_player_map;

pub async fn handle_live(gw: Option<u32>, limit: usize) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let player_map = create_player_map(&bootstrap_data.elements);

    let event_id = match get_effective_event_id(&bootstrap_data.events, gw) {
        Some(id) => id,
        None => {
            println!("Could not determine current Gameweek.");
            return Ok(());
        }
    };

    let (data, event_id) = match FplClient::fetch_live(event_id).await {
        Ok(data) if data.elements.is_empty() && gw.is_none() && event_id > 1 => {
            println!(
                "No live data available for GW {}. Falling back to GW {}.",
                event_id,
                event_id - 1
            );
            (FplClient::fetch_live(event_id - 1).await?, event_id - 1)
        }
        Ok(data) => (data, event_id),
        Err(e) => return Err(e),
    };

    if data.elements.is_empty() {
        println!(
            "No live data available for GW {}. The matches might not have started yet.",
            event_id
        );
        return Ok(());
    }

    let mut elements = data.elements;
    elements.sort_by(|a, b| b.stats.total_points.cmp(&a.stats.total_points));

    println!("Live Stats for GW{}", event_id);

    println!(
        "{:>id_w$}  {:<name_w$}  {:>pts_w$}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}",
        "ID",
        "Name",
        "Pts",
        "Min",
        "G",
        "A",
        "CS",
        "GC",
        "S",
        "PS",
        "PM",
        "YC",
        "RC",
        "OG",
        "B",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pts_w = WIDTH_PTS,
    );
    for element in elements.iter().take(limit) {
        let name = player_map
            .get(&element.id)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let mut stats = StatsPoints::default();
        for explain in &element.explain {
            for stat in &explain.stats {
                match stat.identifier.as_str() {
                    "minutes" => stats.minutes += stat.points,
                    "goals_scored" => stats.goals_scored += stat.points,
                    "assists" => stats.assists += stat.points,
                    "clean_sheets" => stats.clean_sheets += stat.points,
                    "goals_conceded" => stats.goals_conceded += stat.points,
                    "saves" => stats.saves += stat.points,
                    "penalties_saved" => stats.penalties_saved += stat.points,
                    "penalties_missed" => stats.penalties_missed += stat.points,
                    "yellow_cards" => stats.yellow_cards += stat.points,
                    "red_cards" => stats.red_cards += stat.points,
                    "own_goals" => stats.own_goals += stat.points,
                    "bonus" => stats.bonus += stat.points,
                    _ => {}
                }
            }
        }

        println!(
            "{:>id_w$}  {:<name_w$}  {:>pts_w$}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}",
            element.id,
            name,
            element.stats.total_points,
            stats.minutes,
            stats.goals_scored,
            stats.assists,
            stats.clean_sheets,
            stats.goals_conceded,
            stats.saves,
            stats.penalties_saved,
            stats.penalties_missed,
            stats.yellow_cards,
            stats.red_cards,
            stats.own_goals,
            stats.bonus,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pts_w = WIDTH_PTS,
        );
    }

    Ok(())
}
