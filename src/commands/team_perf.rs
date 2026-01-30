use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::Result;
use crate::utils::event_helpers::get_current_event_id;
use crate::utils::formatters::color_trend;

pub async fn handle_team_perf(gw: Option<u32>, last: usize) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;

    // Find current gameweek if not specified using helper
    let current_gw = get_current_event_id(&bootstrap_data.events);

    let end_gw = gw.or(current_gw).unwrap_or(1);
    let start_gw = if end_gw > last as u32 {
        end_gw - last as u32 + 1
    } else {
        1
    };

    // Create player -> team mapping
    let player_team_map: HashMap<u64, u64> = bootstrap_data
        .elements
        .iter()
        .map(|e| (e.id, e.team))
        .collect();

    // Calculate season total points per team (from bootstrap_static)
    let mut team_season_points: HashMap<u64, i64> = HashMap::new();
    for team in &bootstrap_data.teams {
        team_season_points.insert(team.id, 0);
    }
    for element in &bootstrap_data.elements {
        *team_season_points.entry(element.team).or_insert(0) += element.total_points;
    }

    // Count finished GWs for season average calculation
    let finished_gw_count = bootstrap_data
        .events
        .iter()
        .filter(|e| e.finished || e.is_current)
        .count() as f64;

    // Collect points per team per GW
    let mut team_gw_points: HashMap<u64, Vec<(u32, i64)>> = HashMap::new();
    for team in &bootstrap_data.teams {
        team_gw_points.insert(team.id, Vec::new());
    }

    // Fetch live data for each GW
    let mut gw_list: Vec<u32> = Vec::new();
    for gw_num in start_gw..=end_gw {
        // Check if this GW has finished or is current
        let event = bootstrap_data.events.iter().find(|e| e.id == gw_num as u64);
        if let Some(ev) = event {
            if !ev.finished && !ev.is_current {
                continue;
            }
        } else {
            continue;
        }

        match FplClient::fetch_live(gw_num).await {
            Ok(live_data) => {
                gw_list.push(gw_num);

                // Aggregate points per team
                let mut team_points: HashMap<u64, i64> = HashMap::new();
                for team in &bootstrap_data.teams {
                    team_points.insert(team.id, 0);
                }

                for element in &live_data.elements {
                    if let Some(&team_id) = player_team_map.get(&element.id) {
                        *team_points.entry(team_id).or_insert(0) += element.stats.total_points;
                    }
                }

                for (team_id, points) in team_points {
                    if let Some(gw_points) = team_gw_points.get_mut(&team_id) {
                        gw_points.push((gw_num, points));
                    }
                }
            }
            Err(_) => {
                // Skip GWs that can't be fetched
                continue;
            }
        }
    }

    if gw_list.is_empty() {
        println!("No gameweek data available.");
        return Ok(());
    }

    // Calculate average and trend for each team
    // (team_id, name, points_vec, avg, min, max, season_avg, trend)
    let mut team_stats: Vec<(u64, String, Vec<i64>, f64, i64, i64, f64, &str)> = Vec::new();

    for team in &bootstrap_data.teams {
        if let Some(gw_points) = team_gw_points.get(&team.id) {
            let points: Vec<i64> = gw_list
                .iter()
                .map(|gw| {
                    gw_points
                        .iter()
                        .find(|(g, _)| g == gw)
                        .map(|(_, p)| *p)
                        .unwrap_or(0)
                })
                .collect();

            let avg = if !points.is_empty() {
                points.iter().sum::<i64>() as f64 / points.len() as f64
            } else {
                0.0
            };

            let min = points.iter().copied().min().unwrap_or(0);
            let max = points.iter().copied().max().unwrap_or(0);

            // Season average
            let season_total = *team_season_points.get(&team.id).unwrap_or(&0);
            let season_avg = if finished_gw_count > 0.0 {
                season_total as f64 / finished_gw_count
            } else {
                0.0
            };

            // Calculate trend (compare last 2 GWs)
            let trend = if points.len() >= 2 {
                let last = points[points.len() - 1];
                let prev = points[points.len() - 2];
                if last > prev {
                    "↑"
                } else if last < prev {
                    "↓"
                } else {
                    "→"
                }
            } else {
                "→"
            };

            team_stats.push((
                team.id,
                team.name.clone(),
                points,
                avg,
                min,
                max,
                season_avg,
                trend,
            ));
        }
    }

    // Sort by average points (descending)
    team_stats.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());

    // Print header
    let mut header = format!("{:<5} {:<4} {:<20}", "Rank", "ID", "Team");
    for gw in &gw_list {
        header.push_str(&format!(" {:>5}", format!("GW{}", gw)));
    }
    let avg_label = format!("Avg({})", gw_list.len());
    header.push_str(&format!(
        " {:>5} {:>5} {:>8} {:>8} {:>5}",
        "Min", "Max", avg_label, "Avg(all)", "Trend"
    ));
    println!("{}", header);

    // Print data
    for (rank, (team_id, name, points, avg, min, max, season_avg, trend)) in
        team_stats.iter().enumerate()
    {
        let mut row = format!("{:<5} {:<4} {:<20}", rank + 1, team_id, name);
        for p in points {
            row.push_str(&format!(" {:>5}", p));
        }
        row.push_str(&format!(
            " {:>5} {:>5} {:>8.1} {:>8.1} ",
            min, max, avg, season_avg
        ));

        let trend_colored = color_trend(&format!("{:>5}", trend));
        println!("{}{}", row, trend_colored);
    }

    Ok(())
}
