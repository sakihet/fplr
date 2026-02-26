use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::event_helpers::{find_next_event, find_prev_event, get_effective_event_id};
use crate::utils::team_helpers::create_team_map;
use crate::utils::{constants::*, formatters::*};
use clap::Args;
use owo_colors::OwoColorize;
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct FixtureArgs {
    /// Specific Gameweek (defaults to current)
    #[arg(short, long, conflicts_with_all = ["next", "prev"])]
    pub gw: Option<u32>,

    /// Next Gameweek
    #[arg(short, long, conflicts_with = "prev")]
    pub next: bool,

    /// Previous Gameweek
    #[arg(short, long)]
    pub prev: bool,

    /// Show team form for each fixture
    #[arg(short, long)]
    pub form: bool,
}

/// Compute per-team total form aggregated from available players.
/// Returns a map of team_id -> total form.
fn compute_team_form(elements: &[crate::models::Element]) -> HashMap<u64, f64> {
    let mut team_total: HashMap<u64, f64> = HashMap::new();

    for player in elements {
        let is_available = player
            .status
            .is_available(player.chance_of_playing_next_round);
        if !is_available {
            continue;
        }

        let form: f64 = player.form.parse().unwrap_or(0.0);
        *team_total.entry(player.team).or_insert(0.0) += form;
    }

    team_total
}

pub async fn handle_fixture(args: FixtureArgs) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_map(&bootstrap_data.teams);

    let event_id = if args.next {
        find_next_event(&bootstrap_data.events)
            .ok_or(FplrError::NoNextEvent)?
            .id
    } else if args.prev {
        find_prev_event(&bootstrap_data.events)
            .ok_or(FplrError::NoNextEvent)?
            .id
    } else {
        get_effective_event_id(&bootstrap_data.events, args.gw).ok_or(FplrError::NoNextEvent)?
            as u64
    };

    let fixtures = FplClient::fetch_fixtures().await?;

    let mut target_fixtures: Vec<_> = fixtures
        .iter()
        .filter(|f| f.event == Some(event_id))
        .collect();

    if target_fixtures.is_empty() {
        println!("No fixtures found for Gameweek {}.", event_id);
        return Ok(());
    }

    target_fixtures.sort_by(|a, b| a.kickoff_time.cmp(&b.kickoff_time));

    // Compute form data only when the --form flag is set
    let form_map = if args.form {
        Some(compute_team_form(&bootstrap_data.elements))
    } else {
        None
    };

    println!("Fixtures for Gameweek {}:", event_id);

    // Width for form columns – must fit "H.Form" / "A.Form" (6 chars) and values like "12.3" (4 chars)
    const FORM_COL: usize = 6;

    if args.form {
        println!(
            "{:>id_w$}  {:<time_w$}  {:<home_w$}  {:<away_w$}  {:<score_w$}  {:>f_w$}  {:>f_w$}",
            "ID",
            "Kickoff Time",
            "Home",
            "Away",
            "Score",
            "H.Form",
            "A.Form",
            id_w = WIDTH_ID,
            time_w = WIDTH_TIME,
            home_w = WIDTH_TEAM_NAME,
            away_w = WIDTH_TEAM_NAME,
            score_w = WIDTH_SCORE,
            f_w = FORM_COL,
        );
    } else {
        println!(
            "{:>id_w$}  {:<time_w$}  {:<home_w$}  {:<away_w$}  {:<score_w$}",
            "ID",
            "Kickoff Time",
            "Home",
            "Away",
            "Score",
            id_w = WIDTH_ID,
            time_w = WIDTH_TIME,
            home_w = WIDTH_TEAM_NAME,
            away_w = WIDTH_TEAM_NAME,
            score_w = WIDTH_SCORE,
        );
    }

    for fixture in target_fixtures {
        let home_team = team_map
            .get(&fixture.team_h)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let away_team = team_map
            .get(&fixture.team_a)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let kickoff = fixture.kickoff_time.as_deref().unwrap_or("");

        let score = if fixture.finished {
            format!(
                "{} - {}",
                fixture.team_h_score.unwrap_or(0),
                fixture.team_a_score.unwrap_or(0)
            )
        } else {
            "-".to_string()
        };

        if let Some(ref fm) = form_map {
            let h_total = fm.get(&fixture.team_h).cloned().unwrap_or(0.0);
            let a_total = fm.get(&fixture.team_a).cloned().unwrap_or(0.0);

            // Format plain values for width, then color the higher one green
            let h_plain = format!("{:>f_w$.1}", h_total, f_w = FORM_COL);
            let a_plain = format!("{:>f_w$.1}", a_total, f_w = FORM_COL);
            let (h_cell, a_cell) = if h_total > a_total {
                (h_plain.green().to_string(), a_plain)
            } else if a_total > h_total {
                (h_plain, a_plain.green().to_string())
            } else {
                (h_plain, a_plain)
            };

            println!(
                "{:>id_w$}  {:<time_w$}  {:<home_w$}  {:<away_w$}  {:<score_w$}  {}  {}",
                fixture.id,
                format_datetime_local(kickoff),
                home_team,
                away_team,
                score,
                h_cell,
                a_cell,
                id_w = WIDTH_ID,
                time_w = WIDTH_TIME,
                home_w = WIDTH_TEAM_NAME,
                away_w = WIDTH_TEAM_NAME,
                score_w = WIDTH_SCORE,
            );
        } else {
            println!(
                "{:>id_w$}  {:<time_w$}  {:<home_w$}  {:<away_w$}  {:<score_w$}",
                fixture.id,
                format_datetime_local(kickoff),
                home_team,
                away_team,
                score,
                id_w = WIDTH_ID,
                time_w = WIDTH_TIME,
                home_w = WIDTH_TEAM_NAME,
                away_w = WIDTH_TEAM_NAME,
                score_w = WIDTH_SCORE,
            );
        }
    }

    Ok(())
}
