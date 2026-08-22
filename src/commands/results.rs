use std::collections::HashMap;

use owo_colors::OwoColorize;

use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::{WIDTH_STR, WIDTH_TEAM_SHORT_NAME};

fn color_score(score: &str, result: char, cell_w: usize) -> String {
    let s = format!("{:>cell_w$}", score, cell_w = cell_w);
    match result {
        'W' => s.green().to_string(),
        'D' => s.yellow().to_string(),
        _ => s.red().to_string(),
    }
}

pub async fn handle_results() -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let fixtures = FplClient::fetch_fixtures().await?;

    let mut team_indices: Vec<usize> = (0..bootstrap_data.teams.len()).collect();
    team_indices.sort_by_key(|&i| bootstrap_data.teams[i].position);
    let teams: Vec<&_> = team_indices
        .iter()
        .map(|&i| &bootstrap_data.teams[i])
        .collect();

    // (team_id, gw) -> Vec<(score "H-A", result 'W'/'D'/'L')>
    let mut result_map: HashMap<(u64, u64), Vec<(String, char)>> = HashMap::new();
    for f in &fixtures {
        if f.finished
            && let (Some(gw), Some(hs), Some(as_)) = (f.event, f.team_h_score, f.team_a_score)
        {
            let score = format!("{}-{}", hs, as_);
            let h_result = if hs > as_ {
                'W'
            } else if hs == as_ {
                'D'
            } else {
                'L'
            };
            let a_result = if as_ > hs {
                'W'
            } else if as_ == hs {
                'D'
            } else {
                'L'
            };
            result_map
                .entry((f.team_h, gw))
                .or_default()
                .push((score.clone(), h_result));
            result_map
                .entry((f.team_a, gw))
                .or_default()
                .push((score, a_result));
        }
    }

    let max_gw: u64 = fixtures.iter().filter_map(|f| f.event).max().unwrap_or(38);

    // Max fixtures any team has per GW (derived from all fixtures, not just finished)
    let mut team_gw_counts: HashMap<(u64, u64), usize> = HashMap::new();
    for f in &fixtures {
        if let Some(gw) = f.event {
            *team_gw_counts.entry((f.team_h, gw)).or_insert(0) += 1;
            *team_gw_counts.entry((f.team_a, gw)).or_insert(0) += 1;
        }
    }
    let mut gw_slots: HashMap<u64, usize> = HashMap::new();
    for (&(_, gw), &count) in &team_gw_counts {
        let entry = gw_slots.entry(gw).or_insert(1);
        if count > *entry {
            *entry = count;
        }
    }

    let label_w = WIDTH_TEAM_SHORT_NAME;
    let cell_w: usize = WIDTH_STR;

    // Header row: GW number for first slot, "+" for extra DGW slots
    print!("{:<label_w$}  ", "");
    for gw in 1..=max_gw {
        let slots = *gw_slots.get(&gw).unwrap_or(&1);
        print!("{:>cell_w$} ", gw, cell_w = cell_w);
        for _ in 1..slots {
            print!("{:>cell_w$} ", "+", cell_w = cell_w);
        }
    }
    println!();

    for team in &teams {
        print!("{:<label_w$}  ", team.short_name);
        for gw in 1..=max_gw {
            let slots = *gw_slots.get(&gw).unwrap_or(&1);
            let results = result_map.get(&(team.id, gw));
            for slot in 0..slots {
                match results.and_then(|v| v.get(slot)) {
                    Some((score, result)) => {
                        print!("{} ", color_score(score, *result, cell_w))
                    }
                    None => print!("{:>cell_w$} ", "", cell_w = cell_w),
                }
            }
        }
        println!();
    }

    Ok(())
}
