use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::models::{Fixture, Team};
use crate::utils::constants::{
    WIDTH_AVG, WIDTH_DATE, WIDTH_DIFFICULTY, WIDTH_FULL_NAME, WIDTH_HA, WIDTH_ID,
};
use crate::utils::formatters::{
    colorize_text_by_difficulty, difficulty_to_stars, format_datetime_local,
};
use crate::utils::team_helpers::{create_team_ref_map, find_team_ids_by_name};

type TeamFdrData<'a> = (u64, &'a Team, Vec<Vec<(String, u8)>>);

pub async fn handle_fixture_difficulty_rating(
    team: Option<String>,
    limit: usize,
    from: Option<u64>,
    sort_by_avg: bool,
) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_ref_map(&bootstrap_data.teams);

    let fixtures = FplClient::fetch_fixtures().await?;
    // No lower bound unless --from is given
    let start_event = from.unwrap_or(0);
    let unfinished_fixtures: Vec<&Fixture> = fixtures
        .iter()
        .filter(|f| !f.finished && f.event.is_some_and(|e| e >= start_event))
        .collect();

    if let Some(start) = from
        && unfinished_fixtures.is_empty()
    {
        println!("No fixtures found from Gameweek {}.", start);
        return Ok(());
    }

    if let Some(team_name) = team {
        let team_ids = find_team_ids_by_name(&bootstrap_data.teams, &team_name);
        if team_ids.is_empty() {
            return Err(FplrError::TeamNotFoundByName(team_name));
        }
        let tid = team_ids[0];
        display_team_fdr(&unfinished_fixtures, tid, limit, &team_map)?;
    } else {
        display_all_teams_fdr(&unfinished_fixtures, limit, &team_map, sort_by_avg);
    }

    Ok(())
}

fn display_team_fdr(
    fixtures: &[&Fixture],
    team_id: u64,
    limit: usize,
    team_map: &HashMap<u64, &Team>,
) -> Result<()> {
    let team = team_map
        .get(&team_id)
        .ok_or(FplrError::TeamNotFound(team_id))?;

    println!("Team: {} ({})", team.name, team.short_name);
    println!(
        "{:<id_w$} {:<date_w$} {:<opp_w$} {:<ha_w$} {:<diff_w$}",
        "GW",
        "Date",
        "Opponent",
        "H/A",
        "Difficulty",
        id_w = WIDTH_ID,
        date_w = WIDTH_DATE,
        opp_w = WIDTH_FULL_NAME,
        ha_w = WIDTH_HA,
        diff_w = WIDTH_DIFFICULTY,
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
            .map(|kt| format_datetime_local(kt))
            .unwrap_or_else(|| "TBD".to_string());
        let event = fixture.event.unwrap_or(0);
        let difficulty_display = difficulty_to_stars(difficulty);

        println!(
            "{:<id_w$} {:<date_w$} {:<opp_w$} {:<ha_w$} {:<diff_w$}",
            event,
            kickoff,
            opponent,
            location,
            difficulty_display,
            id_w = WIDTH_ID,
            date_w = WIDTH_DATE,
            opp_w = WIDTH_FULL_NAME,
            ha_w = WIDTH_HA,
            diff_w = WIDTH_DIFFICULTY,
        );
    }

    Ok(())
}

fn display_all_teams_fdr(
    fixtures: &[&Fixture],
    limit: usize,
    team_map: &HashMap<u64, &Team>,
    sort_by_avg: bool,
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

    // Build FDR data for each team: (team_id, team, Vec<Vec<(display_text, difficulty)>>)
    let mut team_fdr_data: Vec<TeamFdrData> = Vec::new();

    for (team_id, team) in team_map.iter() {
        let mut fdr_values: Vec<Vec<(String, u8)>> = Vec::new();

        for event in &events_to_show {
            let team_fixtures: Vec<_> = fixtures
                .iter()
                .filter(|f| {
                    f.event == Some(*event) && (f.team_h == *team_id || f.team_a == *team_id)
                })
                .collect();

            let mut gw_fixtures = Vec::new();
            for f in team_fixtures {
                let is_home = f.team_h == *team_id;
                let opponent_id = if is_home { f.team_a } else { f.team_h };
                let difficulty = if is_home {
                    f.team_h_difficulty
                } else {
                    f.team_a_difficulty
                };
                let opponent_short = team_map
                    .get(&opponent_id)
                    .map(|t| t.short_name.as_str())
                    .unwrap_or("???");
                let location = if is_home { "H" } else { "A" };
                gw_fixtures.push((format!("{}({})", opponent_short, location), difficulty));
            }
            fdr_values.push(gw_fixtures);
        }

        team_fdr_data.push((*team_id, *team, fdr_values));
    }

    if sort_by_avg {
        team_fdr_data.sort_by(|a, b| {
            let avg = |data: &TeamFdrData| -> f32 {
                let (total, count) = data
                    .2
                    .iter()
                    .flatten()
                    .fold((0.0f32, 0usize), |(s, c), (_, d)| (s + *d as f32, c + 1));
                if count > 0 {
                    total / count as f32
                } else {
                    f32::MAX
                }
            };
            avg(a).partial_cmp(&avg(b)).unwrap()
        });
    } else {
        team_fdr_data.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    }

    // Calculate dynamic column widths based on content
    let mut column_widths: Vec<usize> = vec![6; events_to_show.len()]; // Minimum width for "GWxx"
    for (_, _, fdr_values) in &team_fdr_data {
        for (idx, gw_fixtures) in fdr_values.iter().enumerate() {
            let cell_width = if gw_fixtures.is_empty() {
                1 // "-"
            } else {
                // Calculate total width: sum of text lengths + spaces between fixtures
                gw_fixtures.iter().map(|(t, _)| t.len()).sum::<usize>()
                    + gw_fixtures.len().saturating_sub(1)
            };
            column_widths[idx] = column_widths[idx].max(cell_width);
        }
    }

    // Print header with dynamic widths
    print!("{:<width$}", "Team", width = WIDTH_FULL_NAME);
    for (idx, event) in events_to_show.iter().enumerate() {
        let width = column_widths[idx];
        print!("  {:<width$}", format!("GW{}", event));
    }
    println!("  {:<width$}", "Avg", width = WIDTH_AVG);

    // Print each team's FDR
    for (_, team, fdr_values) in team_fdr_data {
        print!("{:<width$}", team.name, width = WIDTH_FULL_NAME);

        let mut total = 0.0;
        let mut count = 0;

        for (idx, gw_fixtures) in fdr_values.iter().enumerate() {
            let width = column_widths[idx];
            if gw_fixtures.is_empty() {
                print!("  {:>width$}", "-");
            } else {
                let mut display_parts = Vec::new();
                let mut visual_length = 0;
                for (text, difficulty) in gw_fixtures {
                    display_parts.push(colorize_text_by_difficulty(text, *difficulty));
                    visual_length += text.len();
                    total += *difficulty as f32;
                    count += 1;
                }
                // Add spaces between fixtures to visual length
                visual_length += gw_fixtures.len().saturating_sub(1);

                let joined = display_parts.join(" ");
                // Right-pad to match dynamic column width
                let padding = width.saturating_sub(visual_length);
                print!("  {}{}", joined, " ".repeat(padding));
            }
        }

        let avg = if count > 0 {
            format!("{:.1}", total / count as f32)
        } else {
            "-".to_string()
        };
        println!("  {:<width$}", avg, width = WIDTH_AVG);
    }
}
