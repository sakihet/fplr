use crate::api::FplClient;
use crate::error::Result;
use crate::models::TeamTrendSortBy;
use crate::utils::event_helpers::get_current_event_id;
use crate::utils::formatters::to_sparkline;
use futures::future::join_all;
use std::collections::HashMap;

pub async fn handle_team_trend(sort: TeamTrendSortBy, weeks: usize) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;

    let current_gw = get_current_event_id(&bootstrap_data.events);
    let end_gw = current_gw.unwrap_or(1);
    let start_gw = if end_gw > weeks as u32 {
        end_gw - weeks as u32 + 1
    } else {
        1
    };

    let gw_range: Vec<u32> = if end_gw > 0 {
        (start_gw..=end_gw).collect()
    } else {
        Vec::new()
    };

    if gw_range.is_empty() {
        println!("No gameweeks available to calculate trends.");
        return Ok(());
    }

    // Player -> Team map
    let mut player_team_map = HashMap::new();
    for p in &bootstrap_data.elements {
        player_team_map.insert(p.id, p.team);
    }

    // Fetch Live Data
    let mut live_futures = Vec::new();
    for gw in &gw_range {
        live_futures.push(FplClient::fetch_live(*gw));
    }
    let live_results = join_all(live_futures).await;

    // team_id -> GW -> Stats
    // Stats: Pts, BPS, xG, xGC, ICT
    #[derive(Default, Clone)]
    struct TeamStats {
        pts: Vec<i64>,
        bps: Vec<i64>,
        xg: Vec<f64>,
        xgc: Vec<f64>,
        ict: Vec<f64>,
        saves: Vec<i64>,
    }

    let mut team_stats_map: HashMap<u64, TeamStats> = HashMap::new();
    for team in &bootstrap_data.teams {
        team_stats_map.insert(team.id, TeamStats::default());
    }

    for res in live_results.into_iter() {
        let gw_live = match res {
            Ok(data) => data,
            Err(_) => continue,
        };

        // Aggregations for this GW
        let mut gw_pts: HashMap<u64, i64> = HashMap::new();
        let mut gw_bps: HashMap<u64, i64> = HashMap::new();
        let mut gw_xg: HashMap<u64, f64> = HashMap::new();
        let mut gw_xgc: HashMap<u64, f64> = HashMap::new();
        let mut gw_ict: HashMap<u64, f64> = HashMap::new();
        let mut gw_saves: HashMap<u64, i64> = HashMap::new();

        for element in gw_live.elements {
            if let Some(team_id) = player_team_map.get(&element.id) {
                *gw_pts.entry(*team_id).or_insert(0) += element.stats.total_points;
                *gw_bps.entry(*team_id).or_insert(0) += element.stats.bps;

                let el_xg: f64 = element.stats.expected_goals.parse().unwrap_or(0.0);
                let el_xgc: f64 = element.stats.expected_goals_conceded.parse().unwrap_or(0.0);
                let el_ict: f64 = element.stats.ict_index.parse().unwrap_or(0.0);

                *gw_xg.entry(*team_id).or_insert(0.0) += el_xg;
                *gw_xgc.entry(*team_id).or_insert(0.0) += el_xgc;
                *gw_ict.entry(*team_id).or_insert(0.0) += el_ict;
                *gw_saves.entry(*team_id).or_insert(0) += element.stats.saves as i64;
            }
        }

        for team in &bootstrap_data.teams {
            if let Some(stats) = team_stats_map.get_mut(&team.id) {
                stats.pts.push(*gw_pts.get(&team.id).unwrap_or(&0));
                stats.bps.push(*gw_bps.get(&team.id).unwrap_or(&0));
                stats.xg.push(*gw_xg.get(&team.id).unwrap_or(&0.0));
                stats.xgc.push(*gw_xgc.get(&team.id).unwrap_or(&0.0));
                stats.ict.push(*gw_ict.get(&team.id).unwrap_or(&0.0));
                stats.saves.push(*gw_saves.get(&team.id).unwrap_or(&0));
            }
        }
    }

    // Now compute averages and sparklines
    // (team_id, name, avg_pts, avg_xg, avg_xgc, avg_bps, avg_ict, pts_spark, xg_spark, xgc_spark)
    struct TeamRow {
        name: String,
        avg_pts: f64,
        avg_xg: f64,
        avg_xgc: f64,
        avg_bps: f64,
        avg_ict: f64,
        avg_saves: f64,
        pts: Vec<i64>,
        xg: Vec<f64>,
        xgc: Vec<f64>,
        bps: Vec<i64>,
        ict: Vec<f64>,
        saves: Vec<i64>,
    }

    let mut rows: Vec<TeamRow> = Vec::new();
    let num_gws = gw_range.len() as f64;

    for team in &bootstrap_data.teams {
        if let Some(stats) = team_stats_map.get(&team.id) {
            let avg_pts = stats.pts.iter().sum::<i64>() as f64 / num_gws;
            let avg_bps = stats.bps.iter().sum::<i64>() as f64 / num_gws;
            let avg_xg = stats.xg.iter().sum::<f64>() / num_gws;
            let avg_xgc = stats.xgc.iter().sum::<f64>() / num_gws;
            let avg_ict = stats.ict.iter().sum::<f64>() / num_gws;
            let avg_saves = stats.saves.iter().sum::<i64>() as f64 / num_gws;

            rows.push(TeamRow {
                name: team.short_name.clone(),
                avg_pts,
                avg_xg,
                avg_xgc,
                avg_bps,
                avg_ict,
                avg_saves,
                pts: stats.pts.clone(),
                xg: stats.xg.clone(),
                xgc: stats.xgc.clone(),
                bps: stats.bps.clone(),
                ict: stats.ict.clone(),
                saves: stats.saves.clone(),
            });
        }
    }

    // Sort
    rows.sort_by(|a, b| match sort {
        TeamTrendSortBy::Pts => b
            .avg_pts
            .partial_cmp(&a.avg_pts)
            .unwrap_or(std::cmp::Ordering::Equal),
        TeamTrendSortBy::Xg => b
            .avg_xg
            .partial_cmp(&a.avg_xg)
            .unwrap_or(std::cmp::Ordering::Equal),
        TeamTrendSortBy::Xgc => a
            .avg_xgc
            .partial_cmp(&b.avg_xgc)
            .unwrap_or(std::cmp::Ordering::Equal), // Lower is better
        TeamTrendSortBy::Bps => b
            .avg_bps
            .partial_cmp(&a.avg_bps)
            .unwrap_or(std::cmp::Ordering::Equal),
        TeamTrendSortBy::Ict => b
            .avg_ict
            .partial_cmp(&a.avg_ict)
            .unwrap_or(std::cmp::Ordering::Equal),
        TeamTrendSortBy::Saves => b
            .avg_saves
            .partial_cmp(&a.avg_saves)
            .unwrap_or(std::cmp::Ordering::Equal),
    });

    let max_pts = rows
        .iter()
        .flat_map(|r| r.pts.iter())
        .max()
        .copied()
        .unwrap_or(1);
    let max_xg = rows
        .iter()
        .flat_map(|r| r.xg.iter())
        .fold(0.0_f64, |acc, &x| acc.max(x));
    let max_xgc = rows
        .iter()
        .flat_map(|r| r.xgc.iter())
        .fold(0.0_f64, |acc, &x| acc.max(x));
    let max_bps = rows
        .iter()
        .flat_map(|r| r.bps.iter())
        .max()
        .copied()
        .unwrap_or(1);
    let max_ict = rows
        .iter()
        .flat_map(|r| r.ict.iter())
        .fold(0.0_f64, |acc, &x| acc.max(x));
    let max_saves = rows
        .iter()
        .flat_map(|r| r.saves.iter())
        .max()
        .copied()
        .unwrap_or(1);

    let max_xg_int = (max_xg * 100.0) as i64;
    let max_xgc_int = (max_xgc * 100.0) as i64;
    let max_ict_int = (max_ict * 100.0) as i64;

    // Header width
    let spark_w = gw_range.len().max(6);

    // Print table
    println!(
        "{:>4}  {:<4}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:>5}  {:<p_w$}  {:<xg_w$}  {:<xgc_w$}  {:<bps_w$}  {:<ict_w$}  {:<sav_w$}",
        "Rank",
        "Team",
        "Pts",
        "xG",
        "xGC",
        "BPS",
        "ICT",
        "Sav",
        "P-Trnd",
        "xG-Tr",
        "xGC-Tr",
        "BPS-Tr",
        "ICT-Tr",
        "Sav-Tr",
        p_w = spark_w,
        xg_w = spark_w,
        xgc_w = spark_w,
        bps_w = spark_w,
        ict_w = spark_w,
        sav_w = spark_w
    );

    for (i, row) in rows.iter().enumerate() {
        let pts_spark = to_sparkline(&row.pts, max_pts);
        let bps_spark = to_sparkline(&row.bps, max_bps);
        let sav_spark = to_sparkline(&row.saves, max_saves);

        // Convert f64 to i64 for sparkline
        let xg_ints: Vec<i64> = row.xg.iter().map(|&x| (x * 100.0) as i64).collect();
        let xg_spark = to_sparkline(&xg_ints, max_xg_int);

        let xgc_ints: Vec<i64> = row.xgc.iter().map(|&x| (x * 100.0) as i64).collect();
        let xgc_spark = to_sparkline(&xgc_ints, max_xgc_int);

        let ict_ints: Vec<i64> = row.ict.iter().map(|&x| (x * 100.0) as i64).collect();
        let ict_spark = to_sparkline(&ict_ints, max_ict_int);

        println!(
            "{:>4}  {:<4}  {:>5.1}  {:>5.2}  {:>5.2}  {:>5.1}  {:>5.1}  {:>5.1}  {:<p_w$}  {:<xg_w$}  {:<xgc_w$}  {:<bps_w$}  {:<ict_w$}  {:<sav_w$}",
            i + 1,
            row.name,
            row.avg_pts,
            row.avg_xg,
            row.avg_xgc,
            row.avg_bps,
            row.avg_ict,
            row.avg_saves,
            pts_spark,
            xg_spark,
            xgc_spark,
            bps_spark,
            ict_spark,
            sav_spark,
            p_w = spark_w,
            xg_w = spark_w,
            xgc_w = spark_w,
            bps_w = spark_w,
            ict_w = spark_w,
            sav_w = spark_w
        );
    }

    Ok(())
}
