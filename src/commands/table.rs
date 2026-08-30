use std::collections::HashMap;

use owo_colors::OwoColorize;

use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::event_helpers::find_current_event;
use crate::utils::fixture_helpers::{gameweek_progress, is_in_play, is_settled};
use crate::utils::formatters::{
    color_form_result, color_form_result_in_play, color_league_position, format_signed_number,
};

#[derive(Debug, Clone)]
struct MatchResult {
    event: u64,
    kickoff_time: String,
    result: char, // 'W', 'D', or 'L'
    in_play: bool,
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
    in_play: bool,
    form: Vec<(char, bool)>,
}

impl TeamStats {
    fn goal_difference(&self) -> i64 {
        self.goals_for as i64 - self.goals_against as i64
    }
}

pub async fn handle_table(live: bool) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let fixtures = FplClient::fetch_fixtures().await?;

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

    // Calculate statistics from fixtures with a known score
    for fixture in &fixtures {
        let in_play = is_in_play(fixture);

        // Settled matches always count; in-play ones only with --live
        if in_play {
            if !live {
                continue;
            }
        } else if !is_settled(fixture) {
            continue;
        }

        let (Some(h_score), Some(a_score)) = (fixture.team_h_score, fixture.team_a_score) else {
            continue;
        };

        let event = fixture.event.unwrap_or(0);
        let kickoff = fixture.kickoff_time.clone().unwrap_or_default();

        // Home team
        if let Some(h_stats) = stats_map.get_mut(&fixture.team_h) {
            h_stats.played += 1;
            h_stats.goals_for += h_score;
            h_stats.goals_against += a_score;
            h_stats.in_play |= in_play;

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

            if let Some(fixtures_list) = team_fixtures.get_mut(&fixture.team_h) {
                fixtures_list.push(MatchResult {
                    event,
                    kickoff_time: kickoff.clone(),
                    result,
                    in_play,
                });
            }
        }

        // Away team
        if let Some(a_stats) = stats_map.get_mut(&fixture.team_a) {
            a_stats.played += 1;
            a_stats.goals_for += a_score;
            a_stats.goals_against += h_score;
            a_stats.in_play |= in_play;

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

            if let Some(fixtures_list) = team_fixtures.get_mut(&fixture.team_a) {
                fixtures_list.push(MatchResult {
                    event,
                    kickoff_time: kickoff,
                    result,
                    in_play,
                });
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
        let form: Vec<(char, bool)> = matches
            .iter()
            .take(5)
            .map(|m| (m.result, m.in_play))
            .collect();

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

    // Signal that the current gameweek is not settled yet
    if let Some(current) = find_current_event(&bootstrap_data.events) {
        let progress = gameweek_progress(fixtures.iter().filter(|f| f.event == Some(current.id)));
        let (played, total) = (progress.settled, progress.total);

        if played < total {
            println!(
                "{}",
                format!(
                    "GW{} in progress \u{2014} {played}/{total} matches played",
                    current.id
                )
                .dimmed()
            );
        }
    }

    // Display table
    println!(
        "{:<pos_w$} {:<name_w$} {:<p_w$} {:<w_w$} {:<d_w$} {:<l_w$} {:>gf_w$} {:>ga_w$} {:>gd_w$} {:>pts_w$} {:<form_w$}",
        "Pos",
        "Team",
        "P",
        "W",
        "D",
        "L",
        "GF",
        "GA",
        "GD",
        "Pts",
        "Last 5",
        pos_w = WIDTH_RANK,
        name_w = 20,
        p_w = WIDTH_PLAYED,
        w_w = WIDTH_WIN,
        d_w = WIDTH_DRAW,
        l_w = WIDTH_LOSS,
        gf_w = WIDTH_POINTS,
        ga_w = WIDTH_POINTS,
        gd_w = WIDTH_GD,
        pts_w = WIDTH_RANK,
        form_w = 7,
    );

    for (i, team) in teams.iter().enumerate() {
        let pos = i + 1;

        // Color code by position using helper
        let pos_str = color_league_position(pos, WIDTH_RANK);

        // Mark teams whose tally includes a match still in play
        let played_str = if team.in_play {
            format!("{}*", team.played)
        } else {
            team.played.to_string()
        };

        let gd = team.goal_difference();
        let gd_str = format_signed_number(gd);

        // Format form with color coding using helper
        let form_str: String = team
            .form
            .iter()
            .map(|&(result, in_play)| {
                if in_play {
                    color_form_result_in_play(result)
                } else {
                    color_form_result(result)
                }
            })
            .collect();

        println!(
            "{} {:<name_w$} {:<p_w$} {:<w_w$} {:<d_w$} {:<l_w$} {:>gf_w$} {:>ga_w$} {:>gd_w$} {:>pts_w$} {}",
            pos_str,
            team.name,
            played_str,
            team.won,
            team.drawn,
            team.lost,
            team.goals_for,
            team.goals_against,
            gd_str,
            team.points,
            form_str,
            name_w = 20,
            p_w = WIDTH_PLAYED,
            w_w = WIDTH_WIN,
            d_w = WIDTH_DRAW,
            l_w = WIDTH_LOSS,
            gf_w = WIDTH_POINTS,
            ga_w = WIDTH_POINTS,
            gd_w = WIDTH_GD,
            pts_w = WIDTH_RANK,
        );
    }

    if teams.iter().any(|t| t.in_play) {
        println!();
        println!("{}", "* includes a match in play".dimmed());
    }

    Ok(())
}
