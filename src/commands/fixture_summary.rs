use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::constants::{WIDTH_FULL_NAME, WIDTH_STAT_SMALL};
use crate::utils::fixture_helpers::{is_in_play, is_settled};
use crate::utils::team_helpers::create_team_map;
use owo_colors::OwoColorize;
use std::collections::HashMap;

struct MatchPlayerStat {
    name: String,
    points: i64,
    breakdown: String,
}

pub async fn handle_fixture_summary(fixture_id: u64) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let fixtures = FplClient::fetch_fixtures().await?;

    let target_fixture = fixtures
        .iter()
        .find(|f| f.id == fixture_id)
        .ok_or(FplrError::FixtureNotFound(fixture_id))?;

    let event_id = target_fixture
        .event
        .ok_or(FplrError::FixtureNotScheduled(fixture_id))? as u32;

    let live_data = FplClient::fetch_live(event_id).await?;

    let mut element_map = HashMap::new();
    for element in &bootstrap_data.elements {
        element_map.insert(element.id, element);
    }

    let mut home_stats = Vec::new();
    let mut away_stats = Vec::new();
    let mut home_xg = 0.0;
    let mut away_xg = 0.0;

    for live_element in &live_data.elements {
        for explain in &live_element.explain {
            if explain.fixture == fixture_id {
                // Live xG is a gameweek total, not per fixture
                if let Some(element) = element_map.get(&live_element.id) {
                    let xg = live_element
                        .stats
                        .expected_goals
                        .parse::<f64>()
                        .unwrap_or(0.0);
                    if element.team == target_fixture.team_h {
                        home_xg += xg;
                    } else if element.team == target_fixture.team_a {
                        away_xg += xg;
                    }
                }

                let mut points = 0;
                let mut breakdown_parts = Vec::new();

                for stat in &explain.stats {
                    points += stat.points;

                    let identifier = match stat.identifier.as_str() {
                        "minutes" => "Mins",
                        "goals_scored" => "G",
                        "assists" => "A",
                        "clean_sheets" => "CS",
                        "goals_conceded" => "GC",
                        "own_goals" => "OG",
                        "penalties_saved" => "PenSv",
                        "penalties_missed" => "PenM",
                        "yellow_cards" => "YC",
                        "red_cards" => "RC",
                        "saves" => "Svs",
                        "bonus" => "Bonus",
                        "bps" => "BPS",
                        other => other,
                    };

                    breakdown_parts
                        .push(format!("{}({}:{}p)", identifier, stat.value, stat.points));
                }

                let played_or_action = explain
                    .stats
                    .iter()
                    .any(|s| s.identifier != "minutes" || s.value > 0);

                if played_or_action
                    && !breakdown_parts.is_empty()
                    && let Some(element) = element_map.get(&live_element.id)
                {
                    let stat_record = MatchPlayerStat {
                        name: element.web_name.clone(),
                        points,
                        breakdown: breakdown_parts.join(", "),
                    };

                    if element.team == target_fixture.team_h {
                        home_stats.push(stat_record);
                    } else if element.team == target_fixture.team_a {
                        away_stats.push(stat_record);
                    }
                }
            }
        }
    }

    home_stats.sort_by(|a, b| b.points.cmp(&a.points));
    away_stats.sort_by(|a, b| b.points.cmp(&a.points));

    let team_map = create_team_map(&bootstrap_data.teams);
    let home_team_name = team_map
        .get(&target_fixture.team_h)
        .map(|s| s.as_str())
        .unwrap_or("Home");
    let away_team_name = team_map
        .get(&target_fixture.team_a)
        .map(|s| s.as_str())
        .unwrap_or("Away");

    // A team playing twice this gameweek inflates its live xG total
    let event_fixture_count = |team_id: u64| {
        fixtures
            .iter()
            .filter(|f| {
                f.event == target_fixture.event && (f.team_h == team_id || f.team_a == team_id)
            })
            .count()
    };
    let has_dgw = event_fixture_count(target_fixture.team_h) > 1
        || event_fixture_count(target_fixture.team_a) > 1;

    let in_play = is_in_play(target_fixture);
    let kicked_off = is_settled(target_fixture) || in_play;

    println!("Gameweek {} - Fixture ID: {}", event_id, fixture_id);
    let score = if kicked_off {
        format!(
            "{} - {}{}",
            target_fixture.team_h_score.unwrap_or(0),
            target_fixture.team_a_score.unwrap_or(0),
            if in_play { "*" } else { "" }
        )
    } else {
        "-".to_string()
    };
    println!(
        "{} (Home) {} {} (Away)",
        home_team_name.bold(),
        score,
        away_team_name.bold()
    );
    if kicked_off {
        // Align the totals under the score
        let label_w = format!("{} (Home) ", home_team_name).chars().count();
        println!(
            "{:<label_w$}{:.2} - {:.2}",
            "xG",
            home_xg,
            away_xg,
            label_w = label_w
        );
    }
    println!();

    let name_w = WIDTH_FULL_NAME;
    let pts_w = WIDTH_STAT_SMALL;

    println!("=== {} (Home) Players ===", home_team_name);
    println!(
        "{:<name_w$}   {:>pts_w$}   Breakdown",
        "Name",
        "Pts",
        name_w = name_w,
        pts_w = pts_w
    );
    for stat in home_stats {
        println!(
            "{:<name_w$}   {:>pts_w$}   {}",
            stat.name,
            stat.points,
            stat.breakdown,
            name_w = name_w,
            pts_w = pts_w
        );
    }

    println!();
    println!("=== {} (Away) Players ===", away_team_name);
    println!(
        "{:<name_w$}   {:>pts_w$}   Breakdown",
        "Name",
        "Pts",
        name_w = name_w,
        pts_w = pts_w
    );
    for stat in away_stats {
        println!(
            "{:<name_w$}   {:>pts_w$}   {}",
            stat.name,
            stat.points,
            stat.breakdown,
            name_w = name_w,
            pts_w = pts_w
        );
    }

    let mut notes = Vec::new();
    if in_play {
        notes.push("* includes a match in play");
    }
    if kicked_off && has_dgw {
        notes.push("* xG includes both matches due to Double Gameweek (DGW)");
    }
    if !notes.is_empty() {
        println!();
        for note in notes {
            println!("{}", note.dimmed());
        }
    }

    Ok(())
}
