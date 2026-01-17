use owo_colors::OwoColorize;
use std::collections::HashMap;

use crate::api::FplClient;

#[derive(Debug, Default)]
struct TeamStats {
    name: String,
    played: u64,
    won: u64,
    drawn: u64,
    lost: u64,
    goals_for: u64,
    goals_against: u64,
    points: u64,
}

impl TeamStats {
    fn goal_difference(&self) -> i64 {
        self.goals_for as i64 - self.goals_against as i64
    }
}

pub async fn handle_table() {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            // Fetch fixture data
            match FplClient::fetch_fixtures_typed().await {
                Ok(fixtures) => {
                    // Calculate statistics for each team
                    let mut stats_map: HashMap<u64, TeamStats> = HashMap::new();

                    // Initialize all teams
                    for team in &bootstrap_data.teams {
                        stats_map.insert(
                            team.id,
                            TeamStats {
                                name: team.name.clone(),
                                ..Default::default()
                            },
                        );
                    }

                    // Calculate statistics from finished fixtures
                    for fixture in fixtures {
                        if fixture.finished {
                            if let (Some(h_score), Some(a_score)) =
                                (fixture.team_h_score, fixture.team_a_score)
                            {
                                // Home team
                                if let Some(h_stats) = stats_map.get_mut(&fixture.team_h) {
                                    h_stats.played += 1;
                                    h_stats.goals_for += h_score;
                                    h_stats.goals_against += a_score;

                                    if h_score > a_score {
                                        h_stats.won += 1;
                                        h_stats.points += 3;
                                    } else if h_score == a_score {
                                        h_stats.drawn += 1;
                                        h_stats.points += 1;
                                    } else {
                                        h_stats.lost += 1;
                                    }
                                }

                                // Away team
                                if let Some(a_stats) = stats_map.get_mut(&fixture.team_a) {
                                    a_stats.played += 1;
                                    a_stats.goals_for += a_score;
                                    a_stats.goals_against += h_score;

                                    if a_score > h_score {
                                        a_stats.won += 1;
                                        a_stats.points += 3;
                                    } else if a_score == h_score {
                                        a_stats.drawn += 1;
                                        a_stats.points += 1;
                                    } else {
                                        a_stats.lost += 1;
                                    }
                                }
                            }
                        }
                    }

                    // Sort teams by points
                    let mut teams: Vec<TeamStats> = stats_map.into_values().collect();
                    teams.sort_by(|a, b| {
                        b.points
                            .cmp(&a.points)
                            .then_with(|| b.goal_difference().cmp(&a.goal_difference()))
                            .then_with(|| b.goals_for.cmp(&a.goals_for))
                    });

                    // Display table
                    println!(
                        "{:<4} {:<20} {:<4} {:<4} {:<4} {:<4} {:<6} {:<6} {:<5} {:<4}",
                        "Pos", "Team", "P", "W", "D", "L", "GF", "GA", "GD", "Pts"
                    );

                    for (i, team) in teams.iter().enumerate() {
                        let pos = i + 1;

                        // Color code by position
                        let pos_str = match pos {
                            1..=4 => format!("{:<4}", pos).green().to_string(),
                            5..=6 => format!("{:<4}", pos).cyan().to_string(),
                            18..=20 => format!("{:<4}", pos).red().to_string(),
                            _ => format!("{:<4}", pos),
                        };

                        let gd = team.goal_difference();
                        let gd_str = if gd > 0 {
                            format!("+{}", gd)
                        } else {
                            gd.to_string()
                        };

                        println!(
                            "{} {:<20} {:<4} {:<4} {:<4} {:<4} {:<6} {:<6} {:<5} {:<4}",
                            pos_str,
                            team.name,
                            team.played,
                            team.won,
                            team.drawn,
                            team.lost,
                            team.goals_for,
                            team.goals_against,
                            gd_str,
                            team.points
                        );
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
