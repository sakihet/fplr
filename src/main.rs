mod api;
mod models;

use std::collections::HashMap;

use crate::api::FplClient;
use crate::models::{Element, Position, SortBy, Team};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fixture {},
    Gameweek {},
    Live {
        event: u32,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    Player {
        #[arg(short, long, default_value = "points")]
        sort: SortBy,
        #[arg(short, long)]
        position: Option<Position>,
        #[arg(short, long, default_value = "20")]
        limit: usize,
        #[arg(short, long)]
        team: Option<String>,
    },
    #[command(name = "player-summary")]
    PlayerSummary {
        player_id: u64,
    },
    Team {},
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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Gameweek {} => match FplClient::fetch_bootstrap_static().await {
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
        },
        Commands::Live { event, limit } => match FplClient::fetch_bootstrap_static().await {
            Ok(bootstrap_data) => {
                let player_map: HashMap<u64, String> = bootstrap_data
                    .elements
                    .iter()
                    .map(|player| (player.id, player.web_name.clone()))
                    .collect();

                match FplClient::fetch_live(event).await {
                    Ok(data) => {
                        let mut elements = data.elements;
                        elements.sort_by(|a, b| b.stats.total_points.cmp(&a.stats.total_points));

                        println!("{:<4} {:<20} {:<12}", "ID", "Name", "TOTAL_POINTS");
                        for element in elements.iter().take(limit) {
                            let name = player_map
                                .get(&element.id)
                                .map(|s| s.as_str())
                                .unwrap_or("Unknown");

                            println!(
                                "{:<4} {:<20} {:<12}",
                                element.id, name, element.stats.total_points
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
        },
        Commands::Player {
            sort,
            position,
            limit,
            team,
        } => match FplClient::fetch_bootstrap_static().await {
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
        },
        Commands::PlayerSummary { player_id } => {
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
        Commands::Team {} => match FplClient::fetch_bootstrap_static().await {
            Ok(data) => {
                println!("{:<4} {:<20} {:<4}", "ID", "Name", "Short Name");
                for team in data.teams {
                    println!("{:<4} {:<20} {:<4}", team.id, team.name, team.short_name);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Fixture {} => match FplClient::fetch_bootstrap_static().await {
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
                                        let finished =
                                            fixture["finished"].as_bool().unwrap_or(false);

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
                                        id, kickoff_time, home_team, away_team
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
        },
    }
}
