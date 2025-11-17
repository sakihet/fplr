use clap::{Parser, Subcommand, ValueEnum};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Clone, Debug, ValueEnum)]
enum SortBy {
    Points,
    Price,
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::Points
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Player {
        #[arg(short, long, default_value = "points")]
        sort: SortBy,
    },
    Team {},
    Fixture {},
}

async fn fetch_fpl_data() -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://fantasy.premierleague.com/api/bootstrap-static/";
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    Ok(json)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Player { sort } => {
            println!("show players");
            match fetch_fpl_data().await {
                Ok(data) => {
                    if let Some(elements) = data["elements"].as_array() {
                        println!("Found {} players", elements.len());
                        let mut players: Vec<_> = elements
                            .iter()
                            .filter_map(|player| {
                                let price = player["now_cost"].as_u64()?;
                                let points = player["total_points"].as_u64()?;
                                Some((price, points, player))
                            })
                            .collect();

                        match sort {
                            SortBy::Price => players.sort_by(|a, b| b.0.cmp(&a.0)),
                            SortBy::Points => players.sort_by(|a, b| b.1.cmp(&a.1)),
                        }

                        println!(
                            "{:<20} {:<15} {:<8} {:<8}",
                            "Name", "Team", "Price", "Points"
                        );
                        for (_price, points, player) in players.iter().take(20) {
                            if let (Some(web_name), Some(team_code), Some(price)) = (
                                player["web_name"].as_str(),
                                player["team_code"].as_u64(),
                                player["now_cost"].as_u64(),
                            ) {
                                let price_formatted = format!("{:.1}", price as f64 / 10.0);
                                println!(
                                    "{:<20} {:<15} {:<8} {:<8}",
                                    web_name, team_code, price_formatted, points
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Commands::Team {} => {
            println!("show teams");
            match fetch_fpl_data().await {
                Ok(data) => {
                    if let Some(teams) = data["teams"].as_array() {
                        println!("Found {} teams", teams.len());
                        for team in teams {
                            if let (Some(id), Some(name), Some(short_name)) = (
                                team["id"].as_u64(),
                                team["name"].as_str(),
                                team["short_name"].as_str(),
                            ) {
                                println!("{:2}. {} ({})", id, name, short_name);
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
            println!("show fixtures");
            match fetch_fpl_data().await {
                Ok(data) => {
                    if let Some(fixtures) = data["events"].as_array() {
                        println!("Found {} fixtures", fixtures.len());
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
