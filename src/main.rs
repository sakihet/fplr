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
    Player {
        #[arg(short, long, default_value = "points")]
        sort: SortBy,
        #[arg(short, long)]
        position: Option<Position>,
    },
    Team {},
}

fn create_team_map(teams: &[Team]) -> HashMap<u64, String> {
    teams
        .iter()
        .map(|team| (team.id, team.name.clone()))
        .collect()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Gameweek {} => match FplClient::fetch_bootstrap_static().await {
            Ok(data) => {
                println!("{:<4} {:<16} {:<8} {:<8}", "ID", "Name", "Current", "Next");
                for event in data.events {
                    println!(
                        "{:<4} {:<16} {:<8} {:<8}",
                        event.id, event.name, event.is_current, event.is_next
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Player { sort, position } => match FplClient::fetch_bootstrap_static().await {
            Ok(data) => {
                let team_map = create_team_map(&data.teams);

                let mut players: Vec<Element> = data
                    .elements
                    .into_iter()
                    .filter(|player| {
                        if let Some(ref pos) = position {
                            player.element_type == pos.element_type_id() as u64
                        } else {
                            true
                        }
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
                    "{:<4} {:<20} {:<4} {:<16} {:<8} {:<8} {:<8} {:<8}",
                    "ID", "Name", "Pos", "Team", "Cost", "Selected", "Form", "Points"
                );

                for player in players.iter().take(20) {
                    let team_name = team_map
                        .get(&player.team)
                        .map(|s| s.as_str())
                        .unwrap_or("Unknown");

                    println!(
                        "{:<4} {:<20} {:<4} {:<16} {:<8} {:<8} {:<8} {:<8}",
                        player.id,
                        player.web_name,
                        Position::from_element_type_id(player.element_type)
                            .map(|p| p.display_name().to_string())
                            .unwrap_or("N/A".to_string()),
                        team_name,
                        format!("{:.1}", player.now_cost as f64 / 10.0),
                        player.selected_by_percent,
                        player.form,
                        player.total_points
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
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
        Commands::Fixture {} => match FplClient::fetch_fixtures().await {
            Ok(data) => {
                if let Some(events) = data.as_array() {
                    let mut events: Vec<_> = events
                        .iter()
                        .filter_map(|event| {
                            let id = event["id"].as_u64()?;
                            let kickoff_time = event["kickoff_time"].as_str()?;
                            let team_a = event["team_a"].as_u64()?;
                            let team_h = event["team_h"].as_u64()?;
                            let finished = event["finished"].as_bool().unwrap_or(false);

                            if !finished {
                                Some((id, kickoff_time.to_string(), team_a, team_h))
                            } else {
                                None
                            }
                        })
                        .collect();
                    events.sort_by(|a, b| a.1.cmp(&b.1));
                    println!(
                        "{:<4} {:<20} {:<4} vs {:<4}",
                        "ID", "Kickoff Time", "Home", "Away"
                    );
                    if let Some((id, kickoff_time, team_a, team_h)) = events.first() {
                        println!(
                            "{:<4} {:<20} {:<4} vs {:<4}",
                            id, kickoff_time, team_a, team_h
                        );
                    } else {
                        println!("No upcoming fixtures found.");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
    }
}
