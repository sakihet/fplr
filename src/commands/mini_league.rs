use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::formatters::truncate;
use clap::Args;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct MiniLeagueArgs {
    /// League ID
    league_id: u32,
    /// Highlight your entry ID
    #[arg(short, long)]
    entry: Option<u64>,
}

pub async fn handle_mini_league(args: MiniLeagueArgs) -> Result<()> {
    let standings = FplClient::fetch_league_standings(args.league_id).await?;

    println!("League: {}", standings.league.name);
    println!();
    println!(
        "{:>rank_w$}  {:<delta_w$}  {:<name_w$}  {:<name_w$}  {:>pts_w$}  {:>pts_w$}",
        "Rank",
        "ΔRank",
        "Team",
        "Manager",
        "GW Pts",
        "Total",
        rank_w = WIDTH_RANK,
        delta_w = WIDTH_AVAIL,
        name_w = WIDTH_NAME,
        pts_w = WIDTH_POINTS,
    );

    for result in &standings.standings.results {
        let delta = if result.last_rank == 0 {
            format!("{:<width$}", "-", width = WIDTH_AVAIL)
        } else if result.rank < result.last_rank {
            let d = result.last_rank - result.rank;
            format!("{:<width$}", format!("↑{}", d), width = WIDTH_AVAIL)
                .green()
                .to_string()
        } else if result.rank > result.last_rank {
            let d = result.rank - result.last_rank;
            format!("{:<width$}", format!("↓{}", d), width = WIDTH_AVAIL)
                .red()
                .to_string()
        } else {
            format!("{:<width$}", "→", width = WIDTH_AVAIL)
                .dimmed()
                .to_string()
        };

        let team_name = truncate(&result.entry_name, WIDTH_NAME - 1);
        let manager_name = truncate(&result.player_name, WIDTH_NAME - 1);
        let rank_str = format!("{:>width$}", result.rank, width = WIDTH_RANK);
        let gw_str = format!("{:>width$}", result.event_total, width = WIDTH_POINTS);
        let total_str = format!("{:>width$}", result.total, width = WIDTH_POINTS);

        let is_own = args.entry.map(|e| e == result.entry).unwrap_or(false);

        let line = format!(
            "{}  {}  {:<name_w$}  {:<name_w$}  {}  {}",
            rank_str,
            delta,
            team_name,
            manager_name,
            gw_str,
            total_str,
            name_w = WIDTH_NAME,
        );

        if is_own {
            println!("{}", line.bold());
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
