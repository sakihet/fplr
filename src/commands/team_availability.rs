use crate::api::FplClient;
use crate::error::Result;
use crate::models::PlayerStatus;
use crate::utils::constants::WIDTH_TEAM_NAME;
use crate::utils::formatters::to_bar_graph;

#[derive(Debug, Default)]
struct TeamStats {
    name: String,
    available: usize,
    doubtful: usize,
    injured: usize,
    suspended: usize,
    unavailable: usize,
    not_available: usize,
    unknown: usize,
    played: usize,
    unplayed: usize,
    total: usize,
}

pub async fn handle_team_availability() -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;

    let mut team_stats: std::collections::HashMap<u64, TeamStats> = bootstrap_data
        .teams
        .iter()
        .map(|t| {
            (
                t.id,
                TeamStats {
                    name: t.name.clone(),
                    ..Default::default()
                },
            )
        })
        .collect();

    for element in bootstrap_data.elements {
        if let Some(stats) = team_stats.get_mut(&element.team) {
            if element.status != PlayerStatus::Unavailable {
                stats.total += 1;
            }
            if element.minutes > 0 {
                stats.played += 1;
            } else {
                stats.unplayed += 1;
            }
            match element.status {
                PlayerStatus::Available => stats.available += 1,
                PlayerStatus::Doubtful => stats.doubtful += 1,
                PlayerStatus::Injured => stats.injured += 1,
                PlayerStatus::Suspended => stats.suspended += 1,
                PlayerStatus::Unavailable => stats.unavailable += 1,
                PlayerStatus::NotAvailable => stats.not_available += 1,
                PlayerStatus::Unknown => stats.unknown += 1,
            }
        }
    }

    let mut stats_vec: Vec<_> = team_stats.into_values().collect();

    // Sort by availability percentage (ascending)
    stats_vec.sort_by(|a, b| {
        let a_pct = if a.total > 0 {
            a.available as f32 / a.total as f32
        } else {
            1.0
        };
        let b_pct = if b.total > 0 {
            b.available as f32 / b.total as f32
        } else {
            1.0
        };
        b_pct.partial_cmp(&a_pct).unwrap()
    });

    let team_w = WIDTH_TEAM_NAME;

    println!(
        "{:<team_w$}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>7}  {:<10}",
        "Team",
        "Avail",
        "Doub",
        "Inj",
        "Susp",
        "Unav",
        "N/A",
        "Unk",
        "Play",
        "Unpl",
        "Total",
        "Avail%",
        "Status Chart"
    );

    for s in stats_vec {
        let avail_pct = if s.total > 0 {
            (s.available as f32 / s.total as f32) * 100.0
        } else {
            0.0
        };

        let bar = to_bar_graph(avail_pct, 10);

        println!(
            "{:<team_w$}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>6.1}%  {:<10}",
            s.name,
            s.available,
            s.doubtful,
            s.injured,
            s.suspended,
            s.unavailable,
            s.not_available,
            s.unknown,
            s.played,
            s.unplayed,
            s.total,
            avail_pct,
            bar
        );
    }

    Ok(())
}
