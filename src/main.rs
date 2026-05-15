mod api;
mod commands;
mod config;
mod error;
mod models;
mod utils;

use crate::error::{FplrError, Result};
use crate::models::{Position, SortBy, TeamFormSortBy, TeamHaSortBy, TeamSortBy, TeamTrendSortBy};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show player availability (injuries, suspensions, etc.)
    Availability {
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by player name
        #[arg(short, long)]
        name: Option<String>,
        /// Filter by news content
        #[arg(short = 'N', long)]
        news: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<Position>,
        /// Show all players, not just those with issues
        #[arg(short, long)]
        all: bool,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Compare two players side-by-side
    Compare {
        /// First player ID
        id1: u64,
        /// Second player ID
        id2: u64,
    },
    /// Manage configuration
    Config(commands::ConfigArgs),
    /// Show dream team
    DreamTeam {
        /// Specific Gameweek (defaults to current)
        #[arg(short, long)]
        gw: Option<u32>,
    },
    /// Show upcoming fixtures
    Fixture(commands::FixtureArgs),
    /// Show fixture difficulty rating
    #[command(visible_alias = "fdr")]
    FixtureDifficultyRating {
        #[arg(short, long)]
        team: Option<String>,
        #[arg(short, long, default_value = "5")]
        limit: usize,
        #[arg(short, long)]
        all: bool,
    },
    /// Show detailed points summary for a specific fixture
    #[command(name = "fixture-summary")]
    FixtureSummary {
        /// Fixture ID
        id: u64,
    },
    /// Show gameweeks
    Gameweek {},
    /// Show manager's season history
    History(commands::HistoryArgs),
    /// Show live player stats for a specific event
    Live {
        /// Specific Gameweek (defaults to current)
        #[arg(short, long)]
        gw: Option<u32>,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Show a specific manager's team
    Manager {
        /// Manager ID
        manager_id: u64,
        /// Specific Gameweek (defaults to current)
        #[arg(short, long)]
        gw: Option<u32>,
    },
    /// Show my team
    #[command(name = "my-team")]
    MyTeam(commands::MyTeamArgs),
    /// Show a manager's team picks for a specific event
    Pick {
        /// Manager ID (entry ID)
        manager_id: u64,
        /// Event ID
        event_id: u32,
    },
    /// Show regions
    Region {},
    /// Show players
    Player {
        #[arg(short, long, default_value = "points")]
        sort: SortBy,
        #[arg(short, long)]
        position: Option<Position>,
        #[arg(short, long, default_value = "20")]
        limit: usize,
        #[arg(short, long)]
        team: Option<String>,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        region: Option<u64>,
        #[arg(long)]
        min_cost: Option<f64>,
        #[arg(long)]
        max_cost: Option<f64>,
        /// Show only available players
        #[arg(short, long)]
        available: bool,
    },
    /// Show player summary
    #[command(name = "player-summary")]
    PlayerSummary {
        player_id: u64,
        #[arg(short, long)]
        graph: bool,
    },
    /// Show set piece takers (penalties, free kicks, corners)
    #[command(name = "set-piece")]
    SetPiece {
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
    },
    /// Show status
    Status {},
    /// Show mathematically calculated Fixture Swings
    Swing(commands::SwingArgs),
    /// Show league table
    Table {},
    /// Show talisman players
    Talisman {
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
    },
    /// Show teams
    Team {
        #[arg(short, long, default_value = "pos")]
        sort: TeamSortBy,
    },
    /// Show team availability statistics
    #[command(visible_alias = "ta")]
    TeamAvailability,
    /// Show team home/away performance stats
    #[command(name = "team-ha")]
    TeamHa {
        #[arg(short, long, default_value = "hpts")]
        sort: TeamHaSortBy,
    },
    /// Show team form based on total player form
    #[command(name = "team-form")]
    TeamForm {
        #[arg(short, long, default_value = "total")]
        sort: TeamFormSortBy,
    },
    /// Show team performance based on player points per GW
    #[command(name = "team-perf")]
    TeamPerf {
        /// Specific gameweek (defaults to current)
        #[arg(short, long)]
        gw: Option<u32>,
        /// Number of recent gameweeks to show
        #[arg(short, long, default_value = "5")]
        last: usize,
    },
    /// Show form-adjusted fixture difficulty rating
    #[command(name = "fdr-form")]
    FdrForm {
        #[arg(short, long)]
        team: Option<String>,
        #[arg(short, long, default_value = "5")]
        limit: usize,
        #[arg(short, long)]
        all: bool,
    },
    /// Show team performance trends with sparklines
    #[command(name = "team-trend")]
    TeamTrend {
        /// Sort by metric
        #[arg(short, long, default_value = "pts")]
        sort: TeamTrendSortBy,
        /// Number of recent gameweeks to show
        #[arg(short, long, default_value = "5")]
        weeks: usize,
    },
    /// Show top teams in the overall league
    Top {},
    /// Show popular transfers
    Transfer(commands::TransferArgs),
    /// Show player performance trends with sparklines
    Trend {
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<Position>,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
        /// Number of recent gameweeks to show
        #[arg(short, long, default_value = "5")]
        weeks: usize,
        #[arg(long)]
        min_cost: Option<f64>,
        #[arg(long)]
        max_cost: Option<f64>,
    },
    /// Show xA vs Assists analysis (creativity and efficiency)
    Xa {
        /// Sort by metric
        #[arg(short, long, default_value = "xa")]
        sort: crate::models::XaSortBy,
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<Position>,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Show xG vs Goals scored analysis (finishing ability and efficiency)
    Xg {
        /// Sort by metric
        #[arg(short, long, default_value = "xg")]
        sort: crate::models::XgSortBy,
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<Position>,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Show xGC vs Goals Conceded analysis
    Xgc {
        /// Sort by metric
        #[arg(short, long, default_value = "xgc")]
        sort: crate::models::XgcSortBy,
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<Position>,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Show xGI vs Goal Involvements (Actual G + A) analysis
    Xgi {
        /// Sort by metric
        #[arg(short, long, default_value = "xgi")]
        sort: crate::models::XgiSortBy,
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<Position>,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        print_error(&e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();

    match args.commands {
        Commands::Availability {
            team,
            name,
            news,
            position,
            all,
            limit,
        } => commands::handle_availability(team, name, news, position, all, limit).await?,
        Commands::Compare { id1, id2 } => commands::handle_compare(id1, id2).await?,
        Commands::Config(args) => commands::handle_config(args)?,
        Commands::DreamTeam { gw } => commands::handle_dream_team(gw).await?,
        Commands::Fixture(args) => commands::handle_fixture(args).await?,
        Commands::FixtureDifficultyRating { team, limit, all } => {
            commands::handle_fixture_difficulty_rating(team, limit, all).await?
        }
        Commands::FixtureSummary { id } => commands::handle_fixture_summary(id).await?,
        Commands::Gameweek {} => commands::handle_gameweek().await?,
        Commands::History(args) => commands::handle_history(args).await?,
        Commands::Live { gw, limit } => commands::handle_live(gw, limit).await?,
        Commands::Manager { manager_id, gw } => commands::handle_manager(manager_id, gw).await?,
        Commands::MyTeam(args) => commands::handle_my_team(args).await?,
        Commands::Pick {
            manager_id,
            event_id,
        } => commands::handle_pick(manager_id, event_id).await?,
        Commands::Region {} => commands::handle_region().await?,
        Commands::Player {
            sort,
            position,
            limit,
            team,
            name,
            region,
            min_cost,
            max_cost,
            available,
        } => {
            commands::handle_player(commands::PlayerFilterArgs {
                sort,
                position,
                limit,
                team,
                name,
                region,
                min_cost,
                max_cost,
                available,
            })
            .await?
        }
        Commands::PlayerSummary { player_id, graph } => {
            commands::handle_player_summary(player_id, graph).await?
        }
        Commands::SetPiece { team } => commands::handle_set_piece(team).await?,
        Commands::Status {} => commands::handle_status().await?,
        Commands::Swing(args) => commands::handle_swing(args).await?,
        Commands::Table {} => commands::handle_table().await?,
        Commands::Talisman { team } => commands::handle_talisman(team).await?,
        Commands::Team { sort } => commands::handle_team(&sort).await?,
        Commands::TeamAvailability => commands::handle_team_availability().await?,
        Commands::TeamHa { sort } => commands::handle_team_ha(&sort).await?,
        Commands::TeamForm { sort } => commands::handle_team_form(&sort).await?,
        Commands::TeamPerf { gw, last } => commands::handle_team_perf(gw, last).await?,
        Commands::FdrForm { team, limit, all } => {
            commands::handle_fdr_form(team, limit, all).await?
        }
        Commands::TeamTrend { sort, weeks } => commands::handle_team_trend(sort, weeks).await?,
        Commands::Top {} => commands::handle_top().await?,
        Commands::Transfer(args) => commands::handle_transfer(args).await?,
        Commands::Trend {
            team,
            position,
            limit,
            weeks,
            min_cost,
            max_cost,
        } => commands::handle_trend(team, position, limit, weeks, min_cost, max_cost).await?,
        Commands::Xa {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xa(sort, team, position, limit).await?,
        Commands::Xg {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xg(sort, team, position, limit).await?,
        Commands::Xgc {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xgc(sort, team, position, limit).await?,
        Commands::Xgi {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xgi(sort, team, position, limit).await?,
    }
    Ok(())
}

fn print_error(e: &FplrError) {
    eprintln!("Error: {}", e);

    // Print additional hints for common errors
    match e {
        FplrError::ManagerIdNotSet => {
            eprintln!("Hint: Run `fplr config set manager-id <ID>` to set your manager ID");
        }
        FplrError::Api(_) => {
            eprintln!("Hint: Check your internet connection or try again later");
        }
        _ => {}
    }
}
