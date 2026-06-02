use std::collections::HashMap;

use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::{WIDTH_POINTS, WIDTH_RANK, WIDTH_TEAM_SHORT_NAME};
use crate::utils::formatters::{color_league_position, format_signed_number};

pub async fn handle_team_fpl_rank() -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;

    let mut fpl_pts: HashMap<u64, i64> = HashMap::new();
    for e in &bootstrap_data.elements {
        *fpl_pts.entry(e.team).or_insert(0) += e.total_points;
    }

    let mut teams: Vec<_> = bootstrap_data.teams.iter().collect();
    teams.sort_by(|a, b| {
        fpl_pts
            .get(&b.id)
            .unwrap_or(&0)
            .cmp(fpl_pts.get(&a.id).unwrap_or(&0))
    });

    println!(
        "{:>rank_w$}  {:<name_w$}  {:>pts_w$}  {:>rank_w$}  {:>5}",
        "FPLRk",
        "Team",
        "FPLPts",
        "PLPos",
        "Diff",
        rank_w = WIDTH_RANK,
        name_w = WIDTH_TEAM_SHORT_NAME,
        pts_w = WIDTH_POINTS,
    );

    for (i, team) in teams.iter().enumerate() {
        let fpl_rank = i + 1;
        let pl_pos = team.position as usize;
        let pts = fpl_pts.get(&team.id).copied().unwrap_or(0);
        let diff = pl_pos as i64 - fpl_rank as i64;

        println!(
            "{}  {:<name_w$}  {:>pts_w$}  {}  {:>5}",
            color_league_position(fpl_rank, WIDTH_RANK),
            team.short_name,
            pts,
            color_league_position(pl_pos, WIDTH_RANK),
            format_signed_number(diff),
            name_w = WIDTH_TEAM_SHORT_NAME,
            pts_w = WIDTH_POINTS,
        );
    }

    Ok(())
}
