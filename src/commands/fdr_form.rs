use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::models::{BootstrapStatic, Fixture, Team};
use crate::utils::constants::{
    WIDTH_AVG, WIDTH_DATE, WIDTH_FDR_CELL, WIDTH_FDR_STAT, WIDTH_FORM_DIFF, WIDTH_FULL_NAME,
    WIDTH_HA, WIDTH_ID, WIDTH_STAT_WIDE,
};
use crate::utils::formatters::{colorize_text_by_difficulty, format_datetime_local};
use crate::utils::team_helpers::{create_team_ref_map, find_team_ids_by_name};
use owo_colors::OwoColorize;
use std::collections::HashMap;

pub async fn handle_fdr_form(
    team: Option<String>,
    limit: usize,
    from: Option<u64>,
    _all: bool,
) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_ref_map(&bootstrap_data.teams);
    let fixtures = FplClient::fetch_fixtures().await?;

    // No lower bound unless --from is given
    let start_event = from.unwrap_or(0);

    if let Some(start) = from
        && !fixtures
            .iter()
            .any(|f| !f.finished && f.event.is_some_and(|e| e >= start_event))
    {
        println!("No fixtures found from Gameweek {}.", start);
        return Ok(());
    }

    let team_forms = calculate_team_forms(&fixtures, &bootstrap_data);

    if let Some(team_name) = team {
        let team_ids = find_team_ids_by_name(&bootstrap_data.teams, &team_name);
        if team_ids.is_empty() {
            return Err(FplrError::TeamNotFoundByName(team_name));
        }
        let tid = team_ids[0];
        display_team_fdr_form(&fixtures, tid, limit, start_event, &team_map, &team_forms)?;
    } else {
        display_all_teams_fdr_form(&fixtures, limit, start_event, &team_map, &team_forms);
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct TeamFormEntry {
    results: String, // e.g. "WWDLW"
    total_form: f64, // Sum of player forms from team-form command logic
    form_75: f64,    // Sum of forms for players with 75% chance of playing
}

fn calculate_team_forms(
    fixtures: &[Fixture],
    bootstrap: &BootstrapStatic,
) -> HashMap<u64, TeamFormEntry> {
    let mut team_total_forms: HashMap<u64, f64> = HashMap::new();
    let mut team_75_forms: HashMap<u64, f64> = HashMap::new();
    for player in &bootstrap.elements {
        if !player
            .status
            .is_available(player.chance_of_playing_next_round)
        {
            // Even if not fully available, check if the chance is 75% for team_75_forms
            if player.chance_of_playing_next_round == Some(75) {
                let form: f64 = player.form.parse().unwrap_or(0.0);
                *team_75_forms.entry(player.team).or_insert(0.0) += form;
            }
            continue;
        }
        let form: f64 = player.form.parse().unwrap_or(0.0);
        *team_total_forms.entry(player.team).or_insert(0.0) += form;

        // If the chance is 75% for available players (though usually 100% if is_available returns true)
        if player.chance_of_playing_next_round == Some(75) {
            *team_75_forms.entry(player.team).or_insert(0.0) += form;
        }
    }

    let mut team_forms = HashMap::new();
    let finished_fixtures: Vec<&Fixture> = fixtures.iter().filter(|f| f.finished).collect();

    for team in &bootstrap.teams {
        let mut team_fixtures: Vec<_> = finished_fixtures
            .iter()
            .filter(|f| f.team_h == team.id || f.team_a == team.id)
            .copied()
            .collect();

        // Sort by kickoff time descending to get the latest matches
        team_fixtures.sort_by(|a, b| b.kickoff_time.cmp(&a.kickoff_time));

        let mut results = String::new();
        // Take last 5 matches and reverse them to show in chronological order (left to right)
        for f in team_fixtures.iter().take(5).rev() {
            let is_home = f.team_h == team.id;
            let team_score = if is_home {
                f.team_h_score
            } else {
                f.team_a_score
            };
            let opp_score = if is_home {
                f.team_a_score
            } else {
                f.team_h_score
            };

            if let (Some(ts), Some(os)) = (team_score, opp_score) {
                if ts > os {
                    results.push('W');
                } else if ts == os {
                    results.push('D');
                } else {
                    results.push('L');
                }
            }
        }
        let total_form = *team_total_forms.get(&team.id).unwrap_or(&0.0);
        let form_75 = *team_75_forms.get(&team.id).unwrap_or(&0.0);
        team_forms.insert(
            team.id,
            TeamFormEntry {
                results,
                total_form,
                form_75,
            },
        );
    }
    team_forms
}

/// Revised adjustment based on raw form value (same logic as before but with form values)
/// Average team form is typically around 30-50.
fn get_fdr_adjustment(opp_total_form: f64, avg_form: f64) -> f32 {
    let diff = opp_total_form - avg_form;
    if diff > 15.0 {
        1.0
    } else if diff > 5.0 {
        0.5
    } else if diff > -5.0 {
        0.0
    } else if diff > -15.0 {
        -0.5
    } else {
        -1.0
    }
}

fn colorize_form_string(form: &str) -> String {
    let mut colored = String::new();
    for c in form.chars() {
        match c {
            'W' => colored.push_str(&"W".green().to_string()),
            'D' => colored.push_str(&"D".yellow().to_string()),
            'L' => colored.push_str(&"L".red().to_string()),
            _ => colored.push(c),
        }
    }
    colored
}

fn colorize_form_diff(diff: f64) -> String {
    let threshold = 5.0;
    let s = if diff >= threshold {
        format!("+{:.1}", diff).green().to_string()
    } else if diff <= -threshold {
        format!("{:.1}", diff).red().to_string()
    } else {
        let prefix = if diff > 0.0 { "+" } else { "" };
        format!("{}{:.1}", prefix, diff)
    };
    format!("[{}]", s)
}

fn display_team_fdr_form(
    fixtures: &[Fixture],
    team_id: u64,
    limit: usize,
    start_event: u64,
    team_map: &HashMap<u64, &Team>,
    team_forms: &HashMap<u64, TeamFormEntry>,
) -> Result<()> {
    let team = team_map
        .get(&team_id)
        .ok_or(FplrError::TeamNotFound(team_id))?;

    let team_form_entry = team_forms.get(&team_id).cloned().unwrap_or(TeamFormEntry {
        results: "-----".to_string(),
        total_form: 0.0,
        form_75: 0.0,
    });

    let avg_league_form =
        team_forms.values().map(|v| v.total_form).sum::<f64>() / team_forms.len() as f64;

    println!("\nTeam: {} ({})", team.name.bold(), team.short_name);
    println!(
        "Form Value: {:.1} (75%: {:.1}) | Results: {}",
        team_form_entry.total_form.bold().cyan(),
        team_form_entry.form_75.bold().yellow(),
        colorize_form_string(&team_form_entry.results)
    );
    println!(
        "{:<id_w$} {:<date_w$} {:<opp_w$} {:<ha_w$} {:<stat_w$} {:<stat_w$} {:<diff_w$} {:<stat_w$}",
        "GW",
        "Date",
        "Opponent",
        "H/A",
        "Static",
        "Opp Form",
        "Form Diff",
        "Adj FDR",
        id_w = WIDTH_ID,
        date_w = WIDTH_DATE,
        opp_w = WIDTH_FULL_NAME,
        ha_w = WIDTH_HA,
        stat_w = WIDTH_FDR_STAT,
        diff_w = WIDTH_FORM_DIFF,
    );

    let mut team_fixtures: Vec<_> = fixtures
        .iter()
        .filter(|f| {
            !f.finished
                && f.event.is_some_and(|e| e >= start_event)
                && (f.team_h == team_id || f.team_a == team_id)
        })
        .collect();

    team_fixtures.sort_by(|a, b| {
        let a_event = a.event.unwrap_or(0);
        let b_event = b.event.unwrap_or(0);
        a_event
            .cmp(&b_event)
            .then_with(|| a.kickoff_time.cmp(&b.kickoff_time))
    });

    for f in team_fixtures.iter().take(limit) {
        let is_home = f.team_h == team_id;
        let opponent_id = if is_home { f.team_a } else { f.team_h };
        let static_difficulty = if is_home {
            f.team_h_difficulty
        } else {
            f.team_a_difficulty
        };
        let opponent = team_map
            .get(&opponent_id)
            .ok_or(FplrError::TeamNotFound(opponent_id))?;
        let opp_form_entry = team_forms
            .get(&opponent_id)
            .cloned()
            .unwrap_or(TeamFormEntry {
                results: "-----".to_string(),
                total_form: 0.0,
                form_75: 0.0,
            });

        let adjustment = get_fdr_adjustment(opp_form_entry.total_form, avg_league_form);
        let adjusted_fdr = static_difficulty as f32 + adjustment;
        let diff = team_form_entry.total_form - opp_form_entry.total_form;

        let kickoff = f
            .kickoff_time
            .as_ref()
            .map(|kt| format_datetime_local(kt))
            .unwrap_or_else(|| "TBD".to_string());
        let event = f.event.unwrap_or(0);
        let colored_diff = colorize_form_diff(diff);

        // Calculate visible length of colorized diff (e.g. [+12.5] -> 8 visible chars if excluded color)
        // [+12.5] is 8 chars. [0.0] is 5 chars.
        let mut visible_diff_len = 2; // [ ]
        if diff != 0.0 {
            visible_diff_len += 1; // + or -
        }
        visible_diff_len += format!("{:.1}", diff.abs()).len();

        print!(
            "{:<id_w$} {:<date_w$} {:<opp_w$} {:<ha_w$} {:<stat_w$} {:<stat_w$.1} ",
            event,
            kickoff,
            opponent.name,
            if is_home { "H" } else { "A" },
            static_difficulty,
            opp_form_entry.total_form,
            id_w = WIDTH_ID,
            date_w = WIDTH_DATE,
            opp_w = WIDTH_FULL_NAME,
            ha_w = WIDTH_HA,
            stat_w = WIDTH_FDR_STAT,
        );
        print!("{}", colored_diff);
        if WIDTH_FORM_DIFF > visible_diff_len {
            print!("{}", " ".repeat(WIDTH_FORM_DIFF - visible_diff_len));
        }
        println!("{:<stat_w$.1}", adjusted_fdr, stat_w = WIDTH_FDR_STAT);
    }

    Ok(())
}

fn display_all_teams_fdr_form(
    fixtures: &[Fixture],
    limit: usize,
    start_event: u64,
    team_map: &HashMap<u64, &Team>,
    team_forms: &HashMap<u64, TeamFormEntry>,
) {
    let mut events: Vec<u64> = fixtures
        .iter()
        .filter(|f| !f.finished && f.event.is_some_and(|e| e >= start_event))
        .filter_map(|f| f.event)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    events.sort();
    let events_to_show: Vec<u64> = events.iter().take(limit).copied().collect();

    let avg_league_form =
        team_forms.values().map(|v| v.total_form).sum::<f64>() / team_forms.len() as f64;

    let mut team_fdr_data: Vec<_> = Vec::new();

    for (team_id, team) in team_map.iter() {
        let mut row_data = Vec::new();
        let mut total_opp_form = 0.0;
        let mut count = 0;

        let team_form_entry = team_forms.get(team_id).cloned().unwrap_or(TeamFormEntry {
            results: "-----".to_string(),
            total_form: 0.0,
            form_75: 0.0,
        });

        for event in &events_to_show {
            let gw_fixtures: Vec<_> = fixtures
                .iter()
                .filter(|f| {
                    f.event == Some(*event) && (f.team_h == *team_id || f.team_a == *team_id)
                })
                .collect();

            let mut gw_entries = Vec::new();
            for f in gw_fixtures {
                let is_home = f.team_h == *team_id;
                let opponent_id = if is_home { f.team_a } else { f.team_h };
                let static_difficulty = if is_home {
                    f.team_h_difficulty
                } else {
                    f.team_a_difficulty
                };
                let opponent_short = team_map
                    .get(&opponent_id)
                    .map(|t| t.short_name.as_str())
                    .unwrap_or("???");
                let opp_form_entry =
                    team_forms
                        .get(&opponent_id)
                        .cloned()
                        .unwrap_or(TeamFormEntry {
                            results: "-----".to_string(),
                            total_form: 0.0,
                            form_75: 0.0,
                        });

                let adjustment = get_fdr_adjustment(opp_form_entry.total_form, avg_league_form);
                let adjusted_fdr = static_difficulty as f32 + adjustment;
                let diff = team_form_entry.total_form - opp_form_entry.total_form;

                gw_entries.push((
                    format!("{}({})", opponent_short, if is_home { "H" } else { "A" }),
                    adjusted_fdr,
                    diff,
                    opp_form_entry.total_form,
                ));
                total_opp_form += opp_form_entry.total_form;
                count += 1;
            }
            row_data.push(gw_entries);
        }

        let avg_opp_form = if count > 0 {
            total_opp_form / count as f64
        } else {
            0.0
        };

        team_fdr_data.push((
            team.name.clone(),
            team_form_entry.total_form,
            team_form_entry.form_75,
            team_form_entry.results,
            row_data,
            avg_opp_form,
        ));
    }

    team_fdr_data.sort_by(|a, b| a.0.cmp(&b.0));

    // Print Table
    println!("\nForm-Adjusted FDR Matrix (Next {} GWs)", limit);
    print!(
        "{:<name_w$} {:<form_w$} {:<form_w$} {:<stat_w$}",
        "Team",
        "Form",
        "75%",
        "Last 5",
        name_w = WIDTH_FULL_NAME,
        form_w = WIDTH_STAT_WIDE,
        stat_w = WIDTH_FDR_STAT,
    );
    for event in &events_to_show {
        print!(
            "  {:<width$}",
            format!("GW{}", event),
            width = WIDTH_FDR_CELL
        );
    }
    println!("  {:<width$}", "Avg", width = WIDTH_AVG);

    for (name, own_form, form_75, results, row_data, avg_opp_f) in team_fdr_data {
        print!(
            "{:<name_w$} {:<form_w$.1} {:<form_w$.1} {:<stat_w$}",
            name,
            own_form,
            form_75,
            colorize_form_string(&results),
            name_w = WIDTH_FULL_NAME,
            form_w = WIDTH_STAT_WIDE,
            stat_w = WIDTH_FDR_STAT,
        );
        for gw_entries in row_data {
            if gw_entries.is_empty() {
                print!("  {:<width$}", "-", width = WIDTH_FDR_CELL);
            } else {
                let mut cell_text = String::new();
                let mut visible_len = 0;
                for (i, (text, difficulty, diff, _opp_form)) in gw_entries.iter().enumerate() {
                    let colored_opp = colorize_text_by_difficulty(text, difficulty.round() as u8);
                    let colored_diff = colorize_form_diff(*diff);

                    if i > 0 {
                        cell_text.push(' ');
                        visible_len += 1;
                    }
                    cell_text.push_str(&colored_opp);
                    cell_text.push_str(&colored_diff);

                    visible_len += text.len();
                    visible_len += 2; // [ ]
                    if *diff != 0.0 {
                        visible_len += 1; // + or -
                    }
                    visible_len += format!("{:.1}", diff.abs()).len();
                }

                print!("  {}", cell_text);
                if WIDTH_FDR_CELL > visible_len {
                    print!("{}", " ".repeat(WIDTH_FDR_CELL - visible_len));
                }
            }
        }
        println!("  {:<width$.1}", avg_opp_f, width = WIDTH_AVG);
    }
}
