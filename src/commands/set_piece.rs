use crate::api::FplClient;

pub async fn handle_set_piece(team_name: Option<String>) {
    // Fetch bootstrap-static to resolve team names
    let bootstrap = match FplClient::fetch_bootstrap_static().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching teams: {}", e);
            return;
        }
    };

    match FplClient::fetch_set_piece_notes().await {
        Ok(data) => {
            println!("{:<20} {}", "Team", "Set Piece Info");

            for team_notes in data.teams {
                // Get team name from team_id
                let team = bootstrap.teams.iter().find(|t| t.id == team_notes.id);

                let team_full_name = team.map(|t| t.name.as_str()).unwrap_or("Unknown");
                let team_short_name = team.map(|t| t.short_name.as_str()).unwrap_or("");

                // Apply filter if provided
                if let Some(ref filter) = team_name {
                    if !team_short_name
                        .to_lowercase()
                        .contains(&filter.to_lowercase())
                        && !team_full_name
                            .to_lowercase()
                            .contains(&filter.to_lowercase())
                    {
                        continue;
                    }
                }

                // Print each note for this team
                for (i, note) in team_notes.notes.iter().enumerate() {
                    if i == 0 {
                        println!("{:<20} {}", team_full_name, note.info_message);
                    } else {
                        println!("{:<20} {}", "", note.info_message);
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
