use crate::api::FplClient;
use crate::error::Result;

pub async fn handle_team() -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

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

    Ok(())
}
