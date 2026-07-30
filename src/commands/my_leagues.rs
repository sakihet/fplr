use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::EntryLeagueItem;
use crate::utils::constants::*;
use crate::utils::formatters::format_compact_number;
use clap::Args;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct MyLeaguesArgs {
    /// Manager ID (uses configured ID if not provided)
    #[arg(short, long)]
    manager_id: Option<u64>,
}

pub async fn handle_my_leagues(args: MyLeaguesArgs) -> Result<()> {
    let manager_id = if let Some(id) = args.manager_id {
        id
    } else {
        Config::load()?.get_manager_id()?
    };

    let entry = FplClient::fetch_entry_details(manager_id).await?;

    print_league_section("Classic Leagues", &entry.leagues.classic);

    if !entry.leagues.h2h.is_empty() {
        println!();
        print_league_section("H2H Leagues", &entry.leagues.h2h);
    }

    Ok(())
}

fn print_league_section(title: &str, leagues: &[EntryLeagueItem]) {
    println!("=== {} ===", title);

    if leagues.is_empty() {
        println!("No leagues found.");
        return;
    }

    println!(
        "{:>8}  {:<name_w$}  {:<7}  {:>rank_w$}  {:<6}  {:>rank_w$}",
        "ID",
        "Name",
        "Type",
        "Rank",
        "ΔRank",
        "Members",
        name_w = WIDTH_NAME,
        rank_w = WIDTH_RANK_WIDE,
    );

    for league in leagues {
        let type_str = match league.league_type.as_str() {
            "s" => "system",
            "c" => "classic",
            _ => "other",
        };

        let rank_str = if league.entry_rank == 0 {
            format!("{:>width$}", "-", width = WIDTH_RANK_WIDE)
        } else {
            format!(
                "{:>width$}",
                format_compact_number(league.entry_rank),
                width = WIDTH_RANK_WIDE
            )
        };

        let delta = if league.entry_last_rank == 0 || league.entry_rank == 0 {
            format!("{:<6}", "-")
        } else if league.entry_rank < league.entry_last_rank {
            let d = league.entry_last_rank - league.entry_rank;
            format!("{:<6}", format!("↑{}", format_compact_number(d)))
                .green()
                .to_string()
        } else if league.entry_rank > league.entry_last_rank {
            let d = league.entry_rank - league.entry_last_rank;
            format!("{:<6}", format!("↓{}", format_compact_number(d)))
                .red()
                .to_string()
        } else {
            format!("{:<6}", "→").dimmed().to_string()
        };

        let members_str = format_compact_number(league.rank_count);

        println!(
            "{:>8}  {:<name_w$}  {:<7}  {}  {}  {:>rank_w$}",
            league.id,
            crate::utils::formatters::truncate(&league.name, WIDTH_NAME - 1),
            type_str,
            rank_str,
            delta,
            members_str,
            name_w = WIDTH_NAME,
            rank_w = WIDTH_RANK_WIDE,
        );
    }
}
