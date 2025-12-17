use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::api::FplClient;
use crate::models::{Element, Position, SortBy, StatsPoints, Team};

fn format_datetime(datetime_str: &str) -> String {
    let dt = datetime_str.parse::<DateTime<Utc>>().unwrap();
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
}

fn create_team_map(teams: &[Team]) -> HashMap<u64, String> {
    teams
        .iter()
        .map(|team| (team.id, team.name.clone()))
        .collect()
}

fn find_team_ids_by_name(teams: &[Team], name: &str) -> Vec<u64> {
    let search_term = name.to_lowercase();
    teams
        .iter()
        .filter(|team| {
            team.name.to_lowercase().contains(&search_term)
                || team.short_name.to_lowercase().contains(&search_term)
        })
        .map(|team| team.id)
        .collect()
}

fn create_player_map(elements: &[Element]) -> HashMap<u64, String> {
    elements
        .iter()
        .map(|player| (player.id, player.web_name.clone()))
        .collect()
}

pub async fn handle_dream_team(event_id: u32) {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            let player_map = create_player_map(&bootstrap_data.elements);

            match FplClient::fetch_dream_team(event_id).await {
                Ok(data) => {
                    let mut team = data.team;
                    team.sort_by(|a, b| b.points.cmp(&a.points));

                    println!("{:<4} {:<20} {:<12}", "ID", "Name", "Points");
                    for t in team.iter() {
                        let name = player_map
                            .get(&t.element)
                            .map(|s| s.as_str())
                            .unwrap_or("Unknown");

                        println!("{:<4} {:<20} {:<12}", t.element, name, t.points);
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

pub async fn handle_gameweek() {
    match FplClient::fetch_bootstrap_static().await {
        Ok(data) => {
            println!(
                "{:<4} {:<16} {:<12} {:<20}",
                "ID", "Name", "Status", "Deadline"
            );
            for event in data.events {
                let status = if event.is_current {
                    "Current"
                } else if event.is_next {
                    "Next"
                } else if event.finished {
                    "Finished"
                } else {
                    "Upcoming"
                };
                println!(
                    "{:<4} {:<16} {:<12} {:<20}",
                    event.id, event.name, status, event.deadline_time
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

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

pub async fn handle_player(
    sort: SortBy,
    position: Option<Position>,
    limit: usize,
    team: Option<String>,
) {
    match FplClient::fetch_bootstrap_static().await {
        Ok(data) => {
            let team_map = create_team_map(&data.teams);
            let target_team_ids = if let Some(ref team_name) = team {
                find_team_ids_by_name(&data.teams, team_name)
            } else {
                Vec::new()
            };

            let mut players: Vec<Element> = data
                .elements
                .into_iter()
                .filter(|player| {
                    let position_match = if let Some(ref pos) = position {
                        player.element_type == pos.element_type_id() as u64
                    } else {
                        true
                    };
                    let team_match = if team.is_some() {
                        target_team_ids.contains(&player.team)
                    } else {
                        true
                    };
                    position_match && team_match
                })
                .collect();

            match sort {
                SortBy::Cost => players.sort_by(|a, b| b.now_cost.cmp(&a.now_cost)),
                SortBy::Form => players.sort_by(|a, b| {
                    let form_a = a.form.parse::<f64>().unwrap_or(0.0);
                    let form_b = b.form.parse::<f64>().unwrap_or(0.0);
                    form_b.partial_cmp(&form_a).unwrap()
                }),
                SortBy::Points => players.sort_by(|a, b| b.total_points.cmp(&a.total_points)),
                SortBy::SelectedBy => players.sort_by(|a, b| {
                    let selected_by_a = a.selected_by_percent.parse::<f64>().unwrap_or(0.0);
                    let selected_by_b = b.selected_by_percent.parse::<f64>().unwrap_or(0.0);
                    selected_by_b.partial_cmp(&selected_by_a).unwrap()
                }),
            }

            println!(
                "{:<4} {:<20} {:<4} {:<16} {:<8} {:<8} {:<8} {:<8} {:<30}",
                "ID", "Name", "Pos", "Team", "Cost", "Selected", "Form", "Points", "News"
            );

            for player in players.iter().take(limit) {
                let team_name = team_map
                    .get(&player.team)
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");

                println!(
                    "{:<4} {:<20} {:<4} {:<16} {:<8} {:<8} {:<8} {:<8} {:<30}",
                    player.id,
                    player.web_name,
                    Position::from_element_type_id(player.element_type)
                        .map(|p| p.display_name().to_string())
                        .unwrap_or("N/A".to_string()),
                    team_name,
                    format!("{:.1}", player.now_cost as f64 / 10.0),
                    player.selected_by_percent,
                    player.form,
                    player.total_points,
                    player.news,
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub async fn handle_pick(manager_id: u64, event_id: u32) {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            let player_map = create_player_map(&bootstrap_data.elements);

            match FplClient::fetch_live(event_id).await {
                Ok(live_data) => {
                    let points_map: HashMap<u64, i64> = live_data
                        .elements
                        .iter()
                        .map(|element| (element.id, element.stats.total_points))
                        .collect();

                    match FplClient::fetch_manager_picks(manager_id, event_id).await {
                        Ok(picks) => {
                            println!(
                                "{:<4} {:<20} {:<4} {:<4} {:<4} {:<4}",
                                "ID", "Name", "Pos", "C", "VC", "Pts"
                            );
                            for pick in picks.picks.iter() {
                                let name = player_map
                                    .get(&pick.element)
                                    .map(|s| s.as_str())
                                    .unwrap_or("Unknown");

                                let points = points_map.get(&pick.element).copied().unwrap_or(0);

                                println!(
                                    "{:<4} {:<20} {:<4} {:<4} {:<4} {:<4}",
                                    pick.element,
                                    name,
                                    pick.position,
                                    if pick.is_captain { "Y" } else { "N" },
                                    if pick.is_vice_captain { "Y" } else { "N" },
                                    points,
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
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub async fn handle_player_summary(player_id: u64) {
    match FplClient::fetch_player_summary(player_id).await {
        Ok(summary) => {
            let histories = summary.history;
            println!(
                "{:<3} {:<3} {:<4} {:<2} {:<2}",
                "GW", "Pts", "Min", "G", "A"
            );
            for history in histories.iter() {
                println!(
                    "{:<3} {:<3} {:<4} {:<2} {:<2}",
                    history.round,
                    history.total_points,
                    history.minutes,
                    history.goals_scored,
                    history.assists
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub async fn handle_team() {
    match FplClient::fetch_bootstrap_static().await {
        Ok(data) => {
            println!(
                "{:<4} {:<20} {:<8} {:<8}",
                "ID", "Name", "Short", "Strength"
            );
            for team in data.teams {
                println!(
                    "{:<4} {:<20} {:<8} {:<8}",
                    team.id, team.name, team.short_name, team.strength
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub async fn handle_fixture() {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            let team_map = create_team_map(&bootstrap_data.teams);

            if let Some(next_event) = bootstrap_data.events.iter().find(|e| e.is_next) {
                let next_event_id = next_event.id;

                match FplClient::fetch_fixtures().await {
                    Ok(fixtures_data) => {
                        if let Some(fixtures) = fixtures_data.as_array() {
                            let mut next_fixtures: Vec<_> = fixtures
                                .iter()
                                .filter_map(|fixture| {
                                    let event = fixture["event"].as_u64()?;
                                    if event != next_event_id {
                                        return None;
                                    }

                                    let id = fixture["id"].as_u64()?;
                                    let kickoff_time = fixture["kickoff_time"].as_str()?;
                                    let team_a = fixture["team_a"].as_u64()?;
                                    let team_h = fixture["team_h"].as_u64()?;
                                    let finished = fixture["finished"].as_bool().unwrap_or(false);

                                    if !finished {
                                        Some((id, kickoff_time.to_string(), team_a, team_h))
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            next_fixtures.sort_by(|a, b| a.1.cmp(&b.1));
                            println!(
                                "{:<4} {:<20} {:<20} {:<20}",
                                "ID", "Kickoff Time", "Home", "Away"
                            );
                            for (id, kickoff_time, team_h, team_a) in next_fixtures {
                                let home_team = team_map
                                    .get(&team_h)
                                    .map(|s| s.as_str())
                                    .unwrap_or("Unknown");
                                let away_team = team_map
                                    .get(&team_a)
                                    .map(|s| s.as_str())
                                    .unwrap_or("Unknown");
                                println!(
                                    "{:<4} {:<20} {:<20} {:<20}",
                                    id,
                                    format_datetime(&kickoff_time),
                                    home_team,
                                    away_team
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
