use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Player {},
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
        Commands::Player {} => {
            println!("show players");
            match fetch_fpl_data().await {
                Ok(data) => {
                    if let Some(elements) = data["elements"].as_array() {
                        println!("Found {} players", elements.len());
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
                            if let(Some(id), Some(name), Some(short_name)) = (
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
