use crate::api::FplClient;
use crate::utils::formatters::format_datetime;
use crate::utils::team_helpers::create_team_map;

pub async fn handle_fixture() {
    match FplClient::fetch_bootstrap_static().await {
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
                                    let finished = fixture["finished"].as_bool().unwrap_or(false);

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
                                    id,
                                    format_datetime(&kickoff_time),
                                    home_team,
                                    away_team
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
    }
}
