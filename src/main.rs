mod api;
mod models;

use clap::{Parser, Subcommand};
use crate::api::FplClient;
use crate::models::{Position, SortBy};

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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Gameweek {} => match FplClient::fetch_bootstrap_static().await {
            Ok(data) => {
                if let Some(events) = data["events"].as_array() {
                    let events: Vec<_> = events
                        .iter()
                        .filter_map(|event| {
                            let id = event["id"].as_u64()?;
                            let name = event["name"].as_str()?;
                            let is_current = event["is_current"].as_bool()?;
                            let is_next = event["is_next"].as_bool()?;

                            Some((id, name.to_string(), is_current, is_next))
                        })
                        .collect();
                    println!("{:<4} {:<16} {:<8} {:<8}", "ID", "Name", "Current", "Next");
                    for (id, name, is_current, is_next) in events {
                        println!("{:<4} {:<16} {:<8} {:<8}", id, name, is_current, is_next);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Player { sort, position } => match FplClient::fetch_bootstrap_static().await {
            Ok(data) => {
                if let Some(elements) = data["elements"].as_array() {
                    let mut players: Vec<_> = elements
                        .iter()
                        .filter_map(|player| {
                            let id = player["id"].as_u64()?;
                            let cost = player["now_cost"].as_u64()?;
                            let selected_by = player["selected_by_percent"]
                                .as_str()?
                                .parse::<f64>()
                                .ok()?;
                            let form = player["form"].as_str()?.parse::<f64>().ok()?;
                            let points = player["total_points"].as_u64()?;
                            let element_type = player["element_type"].as_u64()? as u8;

                            // filtering by positioin
                            if let Some(ref pos) = position {
                                if element_type != pos.element_type_id() {
                                    return None;
                                }
                            }

                            Some((id, cost, selected_by, form, points, element_type, player))
                        })
                        .collect();

                    match sort {
                        SortBy::Cost => players.sort_by(|a, b| b.1.cmp(&a.1)),
                        SortBy::SelectedBy => {
                            players.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap())
                        }
                        SortBy::Form => players.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap()),
                        SortBy::Points => players.sort_by(|a, b| b.4.cmp(&a.4)),
                    }

                    println!(
                        "{:<4} {:<20} {:<4} {:<16} {:<8} {:<8} {:<8} {:<8}",
                        "ID", "Name", "Pos", "Team", "Cost", "Selected", "Form", "Points"
                    );
                    for (_id, _cost, _selected_by, _form, _points, _element_type, player) in
                        players.iter().take(20)
                    {
                        if let (
                            Some(id),
                            Some(web_name),
                            Some(element_type),
                            Some(team_code),
                            Some(price),
                            Some(selected_by),
                            Some(form),
                            Some(points),
                        ) = (
                            player["id"].as_u64(),
                            player["web_name"].as_str(),
                            player["element_type"].as_u64(),
                            player["team_code"].as_u64(),
                            player["now_cost"].as_u64(),
                            player["selected_by_percent"].as_str(),
                            player["form"].as_str(),
                            player["total_points"].as_u64(),
                        ) {
                            let price_formatted = format!("{:.1}", price as f64 / 10.0);
                            let position_name = Position::from_element_type_id(element_type)
                                .map(|p| p.display_name().to_string())
                                .unwrap_or("N/A".to_string());
                            println!(
                                "{:<4} {:<20} {:<4} {:<16} {:<8} {:<8} {:<8} {:<8}",
                                id,
                                web_name,
                                position_name,
                                team_code,
                                price_formatted,
                                selected_by,
                                form,
                                points
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Team {} => {
            match FplClient::fetch_bootstrap_static().await {
                Ok(data) => {
                    if let Some(teams) = data["teams"].as_array() {
                        println!("{:<4} {:<20} {:<4}", "ID", "Name", "Short Name");
                        for team in teams {
                            if let (Some(id), Some(name), Some(short_name)) = (
                                team["id"].as_u64(),
                                team["name"].as_str(),
                                team["short_name"].as_str(),
                            ) {
                                println!("{:<4} {:<20} {:<4}", id, name, short_name);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Commands::Fixture {} => {
            match FplClient::fetch_fixtures().await {
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
                            println!("{:<4} {:<20} {:<4} vs {:<4}", id, kickoff_time, team_a, team_h);
                        } else {
                            println!("No upcoming fixtures found.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
