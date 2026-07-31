use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::WIDTH_FULL_NAME;

pub async fn handle_set_piece(team_name: Option<String>) -> Result<()> {
    // Fetch bootstrap-static to resolve team names
    let bootstrap = FplClient::fetch_bootstrap_static().await?;
    let data = FplClient::fetch_set_piece_notes().await?;

    println!("{:<width$} Set Piece Info", "Team", width = WIDTH_FULL_NAME);

    for team_notes in data.teams {
        // Get team name from team_id
        let team = bootstrap.teams.iter().find(|t| t.id == team_notes.id);

        let team_full_name = team.map(|t| t.name.as_str()).unwrap_or("Unknown");
        let team_short_name = team.map(|t| t.short_name.as_str()).unwrap_or("");

        // Apply filter if provided
        if let Some(ref filter) = team_name {
            let filter_lower = filter.to_lowercase();
            if !team_short_name.to_lowercase().contains(&filter_lower)
                && !team_full_name.to_lowercase().contains(&filter_lower)
            {
                continue;
            }
        }

        // Print each note for this team
        for (i, note) in team_notes.notes.iter().enumerate() {
            if i == 0 {
                println!(
                    "{:<width$} {}",
                    team_full_name,
                    note.info_message,
                    width = WIDTH_FULL_NAME
                );
            } else {
                println!(
                    "{:<width$} {}",
                    "",
                    note.info_message,
                    width = WIDTH_FULL_NAME
                );
            }
        }
        println!();
    }

    Ok(())
}
