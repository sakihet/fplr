use crate::api::FplClient;
use crate::error::Result;
use crate::models::StatsPoints;
use crate::utils::formatters::*;
use crate::utils::player_helpers::create_player_map;

pub async fn handle_live(event: u32, limit: usize) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let player_map = create_player_map(&bootstrap_data.elements);

    let data = FplClient::fetch_live(event).await?;
    let mut elements = data.elements;
    elements.sort_by(|a, b| b.stats.total_points.cmp(&a.stats.total_points));

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
