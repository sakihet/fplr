use crate::api::FplClient;

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
