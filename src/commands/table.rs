use owo_colors::OwoColorize;
use std::collections::HashMap;

use crate::api::FplClient;

// League position ranges for color coding
const CL_POSITIONS: std::ops::RangeInclusive<usize> = 1..=4; // Champions League
const EL_POSITIONS: std::ops::RangeInclusive<usize> = 5..=6; // Europa League
const RELEGATION_POSITIONS: std::ops::RangeInclusive<usize> = 18..=20;

#[derive(Debug, Clone)]
struct MatchResult {
    event: u64,
    kickoff_time: String,
    result: char, // 'W', 'D', or 'L'
}

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
    form: Vec<char>,
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
            match FplClient::fetch_fixtures().await {
                Ok(fixtures) => {
                    // Calculate statistics for each team
                    let mut stats_map: HashMap<u64, TeamStats> = HashMap::new();
                    let mut team_fixtures: HashMap<u64, Vec<MatchResult>> = HashMap::new();

                    // Initialize all teams
                    for team in &bootstrap_data.teams {
                        stats_map.insert(
                            team.id,
                            TeamStats {
                                name: team.name.clone(),
                                ..Default::default()
                            },
                        );
                        team_fixtures.insert(team.id, Vec::new());
                    }

                    // Calculate statistics from finished fixtures
                    for fixture in &fixtures {
                        if fixture.finished {
                            if let (Some(h_score), Some(a_score)) =
                                (fixture.team_h_score, fixture.team_a_score)
                            {
                                let event = fixture.event.unwrap_or(0);
                                let kickoff = fixture.kickoff_time.clone().unwrap_or_default();

                                // Home team
                                if let Some(h_stats) = stats_map.get_mut(&fixture.team_h) {
                                    h_stats.played += 1;
                                    h_stats.goals_for += h_score;
                                    h_stats.goals_against += a_score;

                                    let result = if h_score > a_score {
                                        h_stats.won += 1;
                                        h_stats.points += 3;
                                        'W'
                                    } else if h_score == a_score {
                                        h_stats.drawn += 1;
                                        h_stats.points += 1;
                                        'D'
                                    } else {
                                        h_stats.lost += 1;
                                        'L'
                                    };

                                    if let Some(fixtures_list) =
                                        team_fixtures.get_mut(&fixture.team_h)
                                    {
                                        fixtures_list.push(MatchResult {
                                            event,
                                            kickoff_time: kickoff.clone(),
                                            result,
                                        });
                                    }
                                }

                                // Away team
                                if let Some(a_stats) = stats_map.get_mut(&fixture.team_a) {
                                    a_stats.played += 1;
                                    a_stats.goals_for += a_score;
                                    a_stats.goals_against += h_score;

                                    let result = if a_score > h_score {
                                        a_stats.won += 1;
                                        a_stats.points += 3;
                                        'W'
                                    } else if a_score == h_score {
                                        a_stats.drawn += 1;
                                        a_stats.points += 1;
                                        'D'
                                    } else {
                                        a_stats.lost += 1;
                                        'L'
                                    };

                                    if let Some(fixtures_list) =
                                        team_fixtures.get_mut(&fixture.team_a)
                                    {
                                        fixtures_list.push(MatchResult {
                                            event,
                                            kickoff_time: kickoff,
                                            result,
                                        });
                                    }
                                }
                            }
                        }
                    }

                    // Calculate recent form (last 5 matches)
                    for (team_id, matches) in team_fixtures.iter_mut() {
                        // Sort by event DESC, then kickoff_time DESC (most recent first)
                        matches.sort_by(|a, b| {
                            b.event
                                .cmp(&a.event)
                                .then_with(|| b.kickoff_time.cmp(&a.kickoff_time))
                        });

                        // Take last 5 matches and collect results
                        let form: Vec<char> = matches.iter().take(5).map(|m| m.result).collect();

                        if let Some(stats) = stats_map.get_mut(team_id) {
                            stats.form = form;
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
                        "{:<4} {:<20} {:<4} {:<4} {:<4} {:<4} {:<6} {:<6} {:<5} {:<4} {:<7}",
                        "Pos", "Team", "P", "W", "D", "L", "GF", "GA", "GD", "Pts", "Last 5"
                    );

                    for (i, team) in teams.iter().enumerate() {
                        let pos = i + 1;

                        // Color code by position
                        let pos_str = if CL_POSITIONS.contains(&pos) {
                            format!("{:<4}", pos).green().to_string()
                        } else if EL_POSITIONS.contains(&pos) {
                            format!("{:<4}", pos).cyan().to_string()
                        } else if RELEGATION_POSITIONS.contains(&pos) {
                            format!("{:<4}", pos).red().to_string()
                        } else {
                            format!("{:<4}", pos)
                        };

                        let gd = team.goal_difference();
                        let gd_str = if gd > 0 {
                            format!("+{}", gd)
                        } else {
                            gd.to_string()
                        };

                        // Format form with color coding
                        let form_str: String = team
                            .form
                            .iter()
                            .map(|&c| match c {
                                'W' => c.to_string().green().to_string(),
                                'L' => c.to_string().red().to_string(),
                                'D' => c.to_string().yellow().to_string(),
                                _ => c.to_string(),
                            })
                            .collect();

                        println!(
                            "{} {:<20} {:<4} {:<4} {:<4} {:<4} {:<6} {:<6} {:<5} {:<4} {}",
                            pos_str,
                            team.name,
                            team.played,
                            team.won,
                            team.drawn,
                            team.lost,
                            team.goals_for,
                            team.goals_against,
                            gd_str,
                            team.points,
                            form_str
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
