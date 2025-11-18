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
    Cost,
    SelectedBy,
    Form,
    Points,
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
        Commands::Player { sort } => match fetch_fpl_data().await {
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
                            Some((id, cost, selected_by, form, points, player))
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
                        "{:<4} {:<20} {:<15} {:<8} {:<8} {:<8} {:<8}",
                        "ID", "Name", "Team", "Cost", "Selected", "Form", "Points"
                    );
                    for (_id, _cost, _selected_by, _form, _points, player) in
                        players.iter().take(20)
                    {
                        if let (
                            Some(id),
                            Some(web_name),
                            Some(team_code),
                            Some(price),
                            Some(selected_by),
                            Some(form),
                            Some(points),
                        ) = (
                            player["id"].as_u64(),
                            player["web_name"].as_str(),
                            player["team_code"].as_u64(),
                            player["now_cost"].as_u64(),
                            player["selected_by_percent"].as_str(),
                            player["form"].as_str(),
                            player["total_points"].as_u64(),
                        ) {
                            let price_formatted = format!("{:.1}", price as f64 / 10.0);
                            println!(
                                "{:<4} {:<20} {:<15} {:<8} {:<8} {:<8} {:<8}",
                                id, web_name, team_code, price_formatted, selected_by, form, points
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
