use std::collections::{HashMap, HashSet};

use futures::future::join_all;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Fixture, TeamStatsSortBy};
use crate::utils::constants::{
    WIDTH_AVG, WIDTH_PLAYED, WIDTH_RANK, WIDTH_STAT, WIDTH_STAT_SMALL, WIDTH_TEAM_SHORT_NAME,
};

const GOALKEEPER_ELEMENT_TYPE: u64 = 1;

/// Column label for the metric the table is sorted by.
fn sort_label(sort_by: &TeamStatsSortBy) -> &'static str {
    match sort_by {
        TeamStatsSortBy::Cs => "CS%",
        TeamStatsSortBy::Ga => "GA/match",
        TeamStatsSortBy::Gf => "GF/match",
        TeamStatsSortBy::Xg => "xG/match",
        TeamStatsSortBy::Xga => "xGA/match",
    }
}

#[derive(Default)]
struct TeamStatsRow {
    name: String,
    played: u32,
    gf: u32,
    ga: u32,
    cs: u32,
    xg: f64,
    xga: f64,
}

impl TeamStatsRow {
    fn per_match(&self, total: f64) -> f64 {
        if self.played == 0 {
            0.0
        } else {
            total / self.played as f64
        }
    }

    fn cs_rate(&self) -> f64 {
        if self.played == 0 {
            0.0
        } else {
            self.cs as f64 / self.played as f64 * 100.0
        }
    }
}

pub async fn handle_team_stats(sort_by: &TeamStatsSortBy, last: Option<usize>) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let fixtures = FplClient::fetch_fixtures().await?;

    // Group fixtures by gameweek so that only fully finished gameweeks are counted.
    // Fixture-derived counts and live xG must cover exactly the same matches, and live
    // data for an in-progress gameweek would include matches not yet reflected in the
    // goal counts.
    let mut gw_fixtures: HashMap<u32, Vec<&Fixture>> = HashMap::new();
    for fixture in &fixtures {
        if let Some(event) = fixture.event {
            gw_fixtures.entry(event as u32).or_default().push(fixture);
        }
    }

    let mut gws: Vec<u32> = gw_fixtures
        .iter()
        .filter(|(_, fs)| fs.iter().all(|f| f.finished))
        .map(|(gw, _)| *gw)
        .collect();
    gws.sort_unstable();

    if let Some(n) = last
        && gws.len() > n
    {
        gws = gws.split_off(gws.len() - n);
    }

    if gws.is_empty() {
        println!("No completed gameweeks available yet.");
        return Ok(());
    }

    let gw_set: HashSet<u32> = gws.iter().copied().collect();

    let mut rows: HashMap<u64, TeamStatsRow> = bootstrap_data
        .teams
        .iter()
        .map(|team| {
            (
                team.id,
                TeamStatsRow {
                    name: team.short_name.clone(),
                    ..Default::default()
                },
            )
        })
        .collect();

    for fixture in &fixtures {
        let Some(event) = fixture.event else { continue };
        if !gw_set.contains(&(event as u32)) {
            continue;
        }
        let h_score = fixture.team_h_score.unwrap_or(0) as u32;
        let a_score = fixture.team_a_score.unwrap_or(0) as u32;

        if let Some(home) = rows.get_mut(&fixture.team_h) {
            home.played += 1;
            home.gf += h_score;
            home.ga += a_score;
            if a_score == 0 {
                home.cs += 1;
            }
        }
        if let Some(away) = rows.get_mut(&fixture.team_a) {
            away.played += 1;
            away.gf += a_score;
            away.ga += h_score;
            if h_score == 0 {
                away.cs += 1;
            }
        }
    }

    let player_team_map: HashMap<u64, u64> = bootstrap_data
        .elements
        .iter()
        .map(|e| (e.id, e.team))
        .collect();
    // A player's expected_goals_conceded covers the xG their team faced while they were
    // on the pitch, so summing every player would multiply it by the size of the squad.
    // Goalkeepers alone give one full match each, and they sum correctly across double
    // gameweeks and mid-match substitutions.
    let goalkeeper_ids: HashSet<u64> = bootstrap_data
        .elements
        .iter()
        .filter(|e| e.element_type == GOALKEEPER_ELEMENT_TYPE)
        .map(|e| e.id)
        .collect();

    let live_results = join_all(gws.iter().map(|gw| FplClient::fetch_live(*gw))).await;

    for live in live_results.into_iter().flatten() {
        for element in live.elements {
            let Some(team_id) = player_team_map.get(&element.id) else {
                continue;
            };
            let Some(row) = rows.get_mut(team_id) else {
                continue;
            };
            row.xg += element.stats.expected_goals.parse::<f64>().unwrap_or(0.0);
            if goalkeeper_ids.contains(&element.id) {
                row.xga += element
                    .stats
                    .expected_goals_conceded
                    .parse::<f64>()
                    .unwrap_or(0.0);
            }
        }
    }

    let mut rows: Vec<TeamStatsRow> = rows.into_values().collect();
    // Sort best-first: more goals and clean sheets, fewer conceded.
    rows.sort_by(|a, b| {
        let ordering = match sort_by {
            TeamStatsSortBy::Cs => b.cs_rate().total_cmp(&a.cs_rate()),
            TeamStatsSortBy::Ga => a
                .per_match(a.ga as f64)
                .total_cmp(&b.per_match(b.ga as f64)),
            TeamStatsSortBy::Gf => b
                .per_match(b.gf as f64)
                .total_cmp(&a.per_match(a.gf as f64)),
            TeamStatsSortBy::Xg => b.per_match(b.xg).total_cmp(&a.per_match(a.xg)),
            TeamStatsSortBy::Xga => a.per_match(a.xga).total_cmp(&b.per_match(b.xga)),
        };
        ordering.then_with(|| a.name.cmp(&b.name))
    });

    let first = gws.first().copied().unwrap_or(0);
    let last_gw = gws.last().copied().unwrap_or(0);
    if first == last_gw {
        println!("GW{} ({} gameweek)", first, gws.len());
    } else {
        println!("GW{}-{} ({} gameweeks)", first, last_gw, gws.len());
    }
    println!("Sorted by {}", sort_label(sort_by));

    println!(
        "{:<rank_w$}  {:<team_w$}  {:>p_w$}  {:>s_w$}  {:>s_w$}  {:>avg_w$}  {:>avg_w$}  {:>s_w$}  {:>avg_w$}  {:>stat_w$}  {:>stat_w$}  {:>avg_w$}  {:>avg_w$}",
        "Rank",
        "Team",
        "P",
        "GF",
        "GA",
        "GF/M",
        "GA/M",
        "CS",
        "CS%",
        "xG",
        "xGA",
        "xG/M",
        "xGA/M",
        rank_w = WIDTH_RANK,
        team_w = WIDTH_TEAM_SHORT_NAME,
        p_w = WIDTH_PLAYED,
        s_w = WIDTH_STAT_SMALL,
        avg_w = WIDTH_AVG,
        stat_w = WIDTH_STAT,
    );

    for (i, row) in rows.iter().enumerate() {
        println!(
            "{:>rank_w$}  {:<team_w$}  {:>p_w$}  {:>s_w$}  {:>s_w$}  {:>avg_w$.2}  {:>avg_w$.2}  {:>s_w$}  {:>avg_w$.1}  {:>stat_w$.2}  {:>stat_w$.2}  {:>avg_w$.2}  {:>avg_w$.2}",
            i + 1,
            row.name,
            row.played,
            row.gf,
            row.ga,
            row.per_match(row.gf as f64),
            row.per_match(row.ga as f64),
            row.cs,
            row.cs_rate(),
            row.xg,
            row.xga,
            row.per_match(row.xg),
            row.per_match(row.xga),
            rank_w = WIDTH_RANK,
            team_w = WIDTH_TEAM_SHORT_NAME,
            p_w = WIDTH_PLAYED,
            s_w = WIDTH_STAT_SMALL,
            avg_w = WIDTH_AVG,
            stat_w = WIDTH_STAT,
        );
    }

    Ok(())
}
