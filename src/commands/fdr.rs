use std::collections::HashMap;

use crate::api::FplClient;
use crate::models::{Fixture, Team};
use crate::utils::formatters::{difficulty_to_stars, format_datetime};

#[allow(dead_code)]
fn difficulty_colored(_difficulty: u8, text: &str) -> String {
    text.to_string()
}

pub async fn handle_fixture_difficulty_rating(
    team_id: Option<u64>,
    limit: usize,
    all_teams: bool,
) {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            let team_map: HashMap<u64, &Team> = bootstrap_data
                .teams
                .iter()
                .map(|team| (team.id, team))
                .collect();

            match FplClient::fetch_fixtures_typed().await {
                Ok(fixtures) => {
                    let unfinished_fixtures: Vec<&Fixture> = fixtures
                        .iter()
                        .filter(|f| !f.finished && f.event.is_some())
                        .collect();

                    if let Some(tid) = team_id {
                        // Show fixtures for a specific team
                        display_team_fdr(&unfinished_fixtures, tid, limit, &team_map);
                    } else if all_teams {
                        // Show FDR matrix for all teams
                        display_all_teams_fdr(&unfinished_fixtures, limit, &team_map);
                    } else {
                        // Default: show all teams
                        display_all_teams_fdr(&unfinished_fixtures, limit, &team_map);
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching fixtures: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching bootstrap data: {}", e);
        }
    }
}

fn display_team_fdr(
    fixtures: &[&Fixture],
    team_id: u64,
    limit: usize,
    team_map: &HashMap<u64, &Team>,
) {
    let team = team_map.get(&team_id);
    if team.is_none() {
        eprintln!("Team not found: {}", team_id);
        return;
    }
    let team = team.unwrap();

    println!("Team: {} ({})", team.name, team.short_name);
    println!(
        "{:<4} {:<20} {:<20} {:<5} {:<15}",
        "GW", "Date", "Opponent", "H/A", "Difficulty"
    );

    let mut team_fixtures: Vec<_> = fixtures
        .iter()
        .filter(|f| f.team_h == team_id || f.team_a == team_id)
        .copied()
        .collect();

    team_fixtures.sort_by(|a, b| {
        let a_event = a.event.unwrap_or(0);
        let b_event = b.event.unwrap_or(0);
        a_event
            .cmp(&b_event)
            .then_with(|| a.kickoff_time.cmp(&b.kickoff_time))
    });

    for fixture in team_fixtures.iter().take(limit) {
        let is_home = fixture.team_h == team_id;
        let opponent_id = if is_home {
            fixture.team_a
        } else {
            fixture.team_h
        };
        let difficulty = if is_home {
            fixture.team_h_difficulty
        } else {
            fixture.team_a_difficulty
        };
        let opponent = team_map
            .get(&opponent_id)
            .map(|t| t.name.as_str())
            .unwrap_or("Unknown");
        let location = if is_home { "H" } else { "A" };
        let kickoff = fixture
            .kickoff_time
            .as_ref()
            .map(|kt| format_datetime(kt))
            .unwrap_or_else(|| "TBD".to_string());
        let event = fixture.event.unwrap_or(0);
        let difficulty_display = difficulty_to_stars(difficulty);

        println!(
            "{:<4} {:<20} {:<20} {:<5} {:<15}",
            event, kickoff, opponent, location, difficulty_display
        );
    }
}

fn display_all_teams_fdr(
    fixtures: &[&Fixture],
    limit: usize,
    team_map: &HashMap<u64, &Team>,
) {
    // Get all unique event IDs and sort them
    let mut events: Vec<u64> = fixtures
        .iter()
        .filter_map(|f| f.event)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    events.sort();
    let events_to_show: Vec<u64> = events.iter().take(limit).copied().collect();

    // Build FDR data for each team
    let mut team_fdr_data: Vec<(u64, &Team, Vec<Option<u8>>)> = Vec::new();

    for (team_id, team) in team_map.iter() {
        let mut fdr_values: Vec<Option<u8>> = Vec::new();

        for event in &events_to_show {
            let team_fixtures: Vec<_> = fixtures
                .iter()
                .filter(|f| f.event == Some(*event) && (f.team_h == *team_id || f.team_a == *team_id))
                .collect();

            if team_fixtures.is_empty() {
                fdr_values.push(None);
            } else {
                // If multiple fixtures, take average (for double gameweeks)
                let avg_difficulty: f32 = team_fixtures
                    .iter()
                    .map(|f| {
                        if f.team_h == *team_id {
                            f.team_h_difficulty as f32
                        } else {
                            f.team_a_difficulty as f32
                        }
                    })
                    .sum::<f32>()
                    / team_fixtures.len() as f32;
                fdr_values.push(Some(avg_difficulty.round() as u8));
            }
        }

        team_fdr_data.push((*team_id, *team, fdr_values));
    }

    // Sort by team name
    team_fdr_data.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    // Print header
    print!("{:<20}", "Team");
    for event in &events_to_show {
        print!(" {:<5}", format!("GW{}", event));
    }
    println!(" {:<5}", "Avg");

    // Print each team's FDR
    for (_, team, fdr_values) in team_fdr_data {
        print!("{:<20}", team.name);

        let mut total = 0.0;
        let mut count = 0;

        for fdr in &fdr_values {
            if let Some(difficulty) = fdr {
                let colored = difficulty_colored(*difficulty, &difficulty.to_string());
                print!(" {:<5}", colored);
                total += *difficulty as f32;
                count += 1;
            } else {
                print!(" {:<5}", "-");
            }
        }

        let avg = if count > 0 {
            format!("{:.1}", total / count as f32)
        } else {
            "-".to_string()
        };
        println!(" {:<5}", avg);
    }
}
