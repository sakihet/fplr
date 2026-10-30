use crate::commands;
use crate::error::Result;
use crate::models::{TeamFormSortBy, TeamHaSortBy, TeamSortBy, TeamStatsSortBy, TeamTrendSortBy};
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct TeamArgs {
    #[command(subcommand)]
    command: TeamCommands,
}

#[derive(Subcommand, Debug)]
enum TeamCommands {
    /// Show team availability statistics
    Availability,
    /// Show team form based on total player form
    Form {
        #[arg(short, long, default_value = "total")]
        sort: TeamFormSortBy,
    },
    /// Show team FPL points rank vs Premier League position
    FplRank {},
    /// Show team home/away performance stats
    Ha {
        #[arg(short, long, default_value = "hpts")]
        sort: TeamHaSortBy,
    },
    /// Show teams
    List {
        #[arg(short, long, default_value = "pos")]
        sort: TeamSortBy,
    },
    /// Show team performance based on player points per GW
    Perf {
        /// Specific gameweek (defaults to current)
        #[arg(short, long)]
        gw: Option<u32>,
        /// Number of recent gameweeks to show
        #[arg(short, long, default_value = "5")]
        last: usize,
    },
    /// Show team attack/defence stats per match (CS%, xG/match)
    Stats {
        /// Sort by metric
        #[arg(short, long, default_value = "xg")]
        sort: TeamStatsSortBy,
        /// Limit to the most recent N completed gameweeks
        #[arg(short, long)]
        last: Option<usize>,
    },
    /// Show team performance trends with sparklines
    Trend {
        /// Sort by metric
        #[arg(short, long, default_value = "pts")]
        sort: TeamTrendSortBy,
        /// Number of recent gameweeks to show
        #[arg(short, long, default_value = "5")]
        weeks: usize,
    },
}

pub async fn run(args: TeamArgs) -> Result<()> {
    match args.command {
        TeamCommands::Availability => commands::handle_team_availability().await?,
        TeamCommands::Form { sort } => commands::handle_team_form(&sort).await?,
        TeamCommands::FplRank {} => commands::handle_team_fpl_rank().await?,
        TeamCommands::Ha { sort } => commands::handle_team_ha(&sort).await?,
        TeamCommands::List { sort } => commands::handle_team(&sort).await?,
        TeamCommands::Perf { gw, last } => commands::handle_team_perf(gw, last).await?,
        TeamCommands::Stats { sort, last } => commands::handle_team_stats(&sort, last).await?,
        TeamCommands::Trend { sort, weeks } => commands::handle_team_trend(sort, weeks).await?,
    }
    Ok(())
}
