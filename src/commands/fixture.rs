use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::event_helpers::get_effective_event_id;
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

    /// Show xG for each fixture (aggregated from player stats)
    #[arg(long)]
    pub xg: bool,
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

    let base_event_id = get_effective_event_id(&bootstrap_data.events, args.gw)
        .ok_or(FplrError::NoNextEvent)? as u32;

    let event_id = if args.next {
        (base_event_id + 1) as u64
    } else if args.prev {
        if base_event_id > 1 {
            (base_event_id - 1) as u64
        } else {
            return Err(FplrError::NoPreviousEvent);
        }
    } else {
        base_event_id as u64
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

    // Count how many matches each team has played/started in this gameweek to average DGW xG
    let mut team_played_counts: HashMap<u64, usize> = HashMap::new();
    for fixture in &target_fixtures {
        if fixture.finished || fixture.started.unwrap_or(false) {
            *team_played_counts.entry(fixture.team_h).or_insert(0) += 1;
            *team_played_counts.entry(fixture.team_a).or_insert(0) += 1;
        }
    }

    // Compute form data only when the --form flag is set
    let form_map = if args.form {
        Some(compute_team_form(&bootstrap_data.elements))
    } else {
        None
    };

    // Compute xG data only when the --xg flag is set
    let mut team_xg: HashMap<u64, f64> = HashMap::new();
    if args.xg {
        let live_data = FplClient::fetch_live(event_id as u32).await?;
        let element_team_map: HashMap<u64, u64> = bootstrap_data
            .elements
            .iter()
            .map(|e| (e.id, e.team))
            .collect();

        for element in live_data.elements {
            let xg: f64 = element.stats.expected_goals.parse().unwrap_or(0.0);
            if let Some(&team_id) = element_team_map.get(&element.id) {
                *team_xg.entry(team_id).or_insert(0.0) += xg;
            }
        }
    }

    println!("Fixtures for Gameweek {}:", event_id);

    // Width for extra columns (Form and xG)
    const EXTRA_COL: usize = 6;

    // Build and print header
    let mut header = format!(
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
    if args.form {
        header.push_str(&format!(
            "  {:>f_w$}  {:>f_w$}",
            "H.Form",
            "A.Form",
            f_w = EXTRA_COL
        ));
    }
    if args.xg {
        header.push_str(&format!(
            "  {:>f_w$}  {:>f_w$}",
            "H.xG",
            "A.xG",
            f_w = EXTRA_COL
        ));
    }
    println!("{}", header);

    let mut has_dgw = false;

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

        let mut row = format!(
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

        if let Some(ref fm) = form_map {
            let h_total = fm.get(&fixture.team_h).cloned().unwrap_or(0.0);
            let a_total = fm.get(&fixture.team_a).cloned().unwrap_or(0.0);

            // Format plain values for width, then color the higher one green
            let h_plain = format!("{:>f_w$.1}", h_total, f_w = EXTRA_COL);
            let a_plain = format!("{:>f_w$.1}", a_total, f_w = EXTRA_COL);
            let (h_cell, a_cell) = if h_total > a_total {
                (h_plain.green().to_string(), a_plain)
            } else if a_total > h_total {
                (h_plain, a_plain.green().to_string())
            } else {
                (h_plain, a_plain)
            };
            row.push_str(&format!("  {}  {}", h_cell, a_cell));
        }

        if args.xg {
            if fixture.finished {
                let mut h_xg = team_xg.get(&fixture.team_h).cloned().unwrap_or(0.0);
                let mut a_xg = team_xg.get(&fixture.team_a).cloned().unwrap_or(0.0);

                let h_count = team_played_counts
                    .get(&fixture.team_h)
                    .cloned()
                    .unwrap_or(1);
                let a_count = team_played_counts
                    .get(&fixture.team_a)
                    .cloned()
                    .unwrap_or(1);

                if h_count > 1 {
                    h_xg /= h_count as f64;
                    has_dgw = true;
                }
                if a_count > 1 {
                    a_xg /= a_count as f64;
                    has_dgw = true;
                }

                let h_marker = if h_count > 1 { "*" } else { "" };
                let a_marker = if a_count > 1 { "*" } else { "" };

                let h_val = format!("{:.2}{}", h_xg, h_marker);
                let a_val = format!("{:.2}{}", a_xg, a_marker);

                let h_plain = format!("{:>f_w$}", h_val, f_w = EXTRA_COL);
                let a_plain = format!("{:>f_w$}", a_val, f_w = EXTRA_COL);
                let (h_cell, a_cell) = if h_xg > a_xg {
                    (h_plain.green().to_string(), a_plain)
                } else if a_xg > h_xg {
                    (h_plain, a_plain.green().to_string())
                } else {
                    (h_plain, a_plain)
                };
                row.push_str(&format!("  {}  {}", h_cell, a_cell));
            } else {
                row.push_str(&format!("  {:>f_w$}  {:>f_w$}", "-", "-", f_w = EXTRA_COL));
            }
        }
        println!("{}", row);
    }

    if args.xg && has_dgw {
        println!("\n* xG is an average per match due to Double Gameweek (DGW)");
    }

    Ok(())
}
