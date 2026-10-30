use crate::models::{Event, Fixture};
use crate::utils::event_helpers::find_next_event;
use crate::utils::formatters::colorize_text_by_difficulty;
use std::collections::HashMap;

/// Progress of a set of fixtures within a gameweek
pub struct GameweekProgress {
    pub total: usize,
    pub settled: usize,
    pub in_play: usize,
    pub awaiting_bonus: usize,
}

/// A fixture's score is final once it is settled.
/// `finished` only flips after FPL confirms bonus points, so `finished_provisional`
/// already marks a settled match.
pub fn is_settled(fixture: &Fixture) -> bool {
    fixture.finished || fixture.finished_provisional
}

/// A fixture is in play once it has kicked off but its score is not settled yet.
pub fn is_in_play(fixture: &Fixture) -> bool {
    fixture.started == Some(true) && !is_settled(fixture)
}

/// A settled fixture still waiting for FPL to confirm bonus points
pub fn is_awaiting_bonus(fixture: &Fixture) -> bool {
    fixture.finished_provisional && !fixture.finished
}

/// Count fixtures by state in a single pass
pub fn gameweek_progress<'a>(fixtures: impl IntoIterator<Item = &'a Fixture>) -> GameweekProgress {
    let mut progress = GameweekProgress {
        total: 0,
        settled: 0,
        in_play: 0,
        awaiting_bonus: 0,
    };

    for fixture in fixtures {
        progress.total += 1;
        if is_settled(fixture) {
            progress.settled += 1;
        }
        if is_in_play(fixture) {
            progress.in_play += 1;
        }
        if is_awaiting_bonus(fixture) {
            progress.awaiting_bonus += 1;
        }
    }

    progress
}

/// Gameweek IDs for the next `weeks` gameweeks, starting at the next gameweek
pub fn upcoming_window(events: &[Event], weeks: u64) -> Vec<u64> {
    let Some(start) = find_next_event(events).map(|e| e.id) else {
        return Vec::new();
    };
    (start..start + weeks)
        .filter(|gw| events.iter().any(|e| e.id == *gw))
        .collect()
}

/// Label a gameweek window such as "GW9-GW11"
pub fn window_label(window: &[u64]) -> String {
    match (window.first(), window.last()) {
        (Some(first), Some(last)) if first == last => format!("GW{}", first),
        (Some(first), Some(last)) => format!("GW{}-GW{}", first, last),
        _ => "none".to_string(),
    }
}

/// Opponents per gameweek in the window; an empty entry means a blank gameweek
pub fn upcoming_fixtures(
    fixtures: &[Fixture],
    team_id: u64,
    window: &[u64],
    team_map: &HashMap<u64, String>,
) -> Vec<Vec<(String, u8)>> {
    window
        .iter()
        .map(|gw| {
            let mut gw_fixtures: Vec<&Fixture> = fixtures
                .iter()
                .filter(|f| f.event == Some(*gw) && (f.team_h == team_id || f.team_a == team_id))
                .collect();
            gw_fixtures.sort_by(|a, b| a.kickoff_time.cmp(&b.kickoff_time));
            gw_fixtures
                .into_iter()
                .map(|f| {
                    let is_home = f.team_h == team_id;
                    let (opponent_id, difficulty) = if is_home {
                        (f.team_a, f.team_h_difficulty)
                    } else {
                        (f.team_h, f.team_a_difficulty)
                    };
                    let opponent = team_map
                        .get(&opponent_id)
                        .map(|s| s.as_str())
                        .unwrap_or("???");
                    let label = format!("{}({})", opponent, if is_home { "H" } else { "A" });
                    (label, difficulty)
                })
                .collect()
        })
        .collect()
}

/// Average difficulty across all fixtures in the window, or None if there are none
pub fn average_difficulty(upcoming: &[Vec<(String, u8)>]) -> Option<f64> {
    let difficulties: Vec<u8> = upcoming.iter().flatten().map(|(_, d)| *d).collect();
    (!difficulties.is_empty())
        .then(|| difficulties.iter().map(|d| *d as f64).sum::<f64>() / difficulties.len() as f64)
}

/// Render upcoming fixtures colored by difficulty; double gameweeks are joined with "+"
pub fn format_upcoming_fixtures(upcoming: &[Vec<(String, u8)>]) -> String {
    upcoming
        .iter()
        .map(|gw| {
            if gw.is_empty() {
                "-".to_string()
            } else {
                gw.iter()
                    .map(|(label, difficulty)| colorize_text_by_difficulty(label, *difficulty))
                    .collect::<Vec<_>>()
                    .join("+")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
