use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::TeamHaSortBy;
use crate::utils::constants::{
    WIDTH_GW, WIDTH_RANK, WIDTH_STAT_SMALL, WIDTH_STR, WIDTH_TEAM_SHORT_NAME,
};

#[derive(Default)]
struct TeamHaStats {
    name: String,
    hp: u32,
    hw: u32,
    hd: u32,
    hl: u32,
    hgs: u32,
    hgc: u32,
    hpts: i64,

    ap: u32,
    aw: u32,
    ad: u32,
    al: u32,
    ags: u32,
    agc: u32,
    apts: i64,

    tpts: i64,
}

pub async fn handle_team_ha(sort_by: &TeamHaSortBy) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let fixtures = FplClient::fetch_fixtures().await?;

    let mut stats_map: HashMap<u64, TeamHaStats> = HashMap::new();
    for team in &bootstrap_data.teams {
        let stats = TeamHaStats {
            name: team.short_name.clone(),
            ..Default::default()
        };
        stats_map.insert(team.id, stats);
    }

    for f in &fixtures {
        if !f.finished {
            continue;
        }
        let h_score = f.team_h_score.unwrap_or(0);
        let a_score = f.team_a_score.unwrap_or(0);

        let h_pts = if h_score > a_score {
            3
        } else if h_score == a_score {
            1
        } else {
            0
        };
        let a_pts = if a_score > h_score {
            3
        } else if h_score == a_score {
            1
        } else {
            0
        };

        if let Some(h_stats) = stats_map.get_mut(&f.team_h) {
            h_stats.hp += 1;
            h_stats.hgs += h_score as u32;
            h_stats.hgc += a_score as u32;
            h_stats.hpts += h_pts;
            if h_pts == 3 {
                h_stats.hw += 1;
            } else if h_pts == 1 {
                h_stats.hd += 1;
            } else {
                h_stats.hl += 1;
            }
            h_stats.tpts += h_pts;
        }

        if let Some(a_stats) = stats_map.get_mut(&f.team_a) {
            a_stats.ap += 1;
            a_stats.ags += a_score as u32;
            a_stats.agc += h_score as u32;
            a_stats.apts += a_pts;
            if a_pts == 3 {
                a_stats.aw += 1;
            } else if a_pts == 1 {
                a_stats.ad += 1;
            } else {
                a_stats.al += 1;
            }
            a_stats.tpts += a_pts;
        }
    }

    let mut stats_vec: Vec<TeamHaStats> = stats_map.into_values().collect();

    stats_vec.sort_by(|a, b| match sort_by {
        TeamHaSortBy::AwayPts => b.apts.cmp(&a.apts).then_with(|| b.tpts.cmp(&a.tpts)),
        TeamHaSortBy::Diff => {
            let diff_a = a.hpts - a.apts;
            let diff_b = b.hpts - b.apts;
            diff_b.cmp(&diff_a).then_with(|| b.tpts.cmp(&a.tpts))
        }
        TeamHaSortBy::HomePts => b.hpts.cmp(&a.hpts).then_with(|| b.tpts.cmp(&a.tpts)),
        TeamHaSortBy::Pts => b.tpts.cmp(&a.tpts).then_with(|| b.hpts.cmp(&a.hpts)),
    });

    println!(
        "{:<rank_w$}  {:<team_w$} | {:>gw_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>stat_w$} | {:>gw_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>stat_w$} | {:>stat_w$} {:>stat_w$}",
        "Rank",
        "Team",
        "HP",
        "HW",
        "HD",
        "HL",
        "HGS",
        "HGC",
        "HPts",
        "AP",
        "AW",
        "AD",
        "AL",
        "AGS",
        "AGC",
        "APts",
        "TPts",
        "Diff",
        rank_w = WIDTH_RANK,
        team_w = WIDTH_TEAM_SHORT_NAME,
        gw_w = WIDTH_GW,
        str_w = WIDTH_STR,
        stat_w = WIDTH_STAT_SMALL,
    );
    for (i, row) in stats_vec.iter().enumerate() {
        let diff = row.hpts - row.apts;
        println!(
            "{:>rank_w$}  {:<team_w$} | {:>gw_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>stat_w$} | {:>gw_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>str_w$} {:>stat_w$} | {:>stat_w$} {:>stat_w$}",
            i + 1,
            row.name,
            row.hp,
            row.hw,
            row.hd,
            row.hl,
            row.hgs,
            row.hgc,
            row.hpts,
            row.ap,
            row.aw,
            row.ad,
            row.al,
            row.ags,
            row.agc,
            row.apts,
            row.tpts,
            if diff > 0 {
                format!("+{}", diff)
            } else {
                format!("{}", diff)
            },
            rank_w = WIDTH_RANK,
            team_w = WIDTH_TEAM_SHORT_NAME,
            gw_w = WIDTH_GW,
            str_w = WIDTH_STR,
            stat_w = WIDTH_STAT_SMALL,
        );
    }
    Ok(())
}
