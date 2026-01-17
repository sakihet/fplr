use crate::api::FplClient;
use crate::models::StatsPoints;
use crate::utils::player_helpers::create_player_map;

pub async fn handle_live(event: u32, limit: usize) {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            let player_map = create_player_map(&bootstrap_data.elements);

            match FplClient::fetch_live(event).await {
                Ok(data) => {
                    let mut elements = data.elements;
                    elements.sort_by(|a, b| b.stats.total_points.cmp(&a.stats.total_points));

                    println!(
                        "{:<4} {:<20} {:<8} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4}",
                        "ID",
                        "Name",
                        "Total",
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
                        "B"
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
                            "{:<4} {:<20} {:<8} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<4}",
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
                            stats.bonus
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
