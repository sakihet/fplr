mod api;
mod commands;
mod config;
mod error;
mod models;
mod utils;

use crate::error::{FplrError, Result};
use crate::models::{Position, SortBy};
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
        /// Show all players, not just those with issues
        #[arg(short, long)]
        all: bool,
        /// Number of players to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Manage configuration
    Config(commands::ConfigArgs),
    /// Show dream team
    DreamTeam { event_id: u32 },
    /// Show upcoming fixtures
    Fixture(commands::FixtureArgs),
    /// Show fixture difficulty rating
    #[command(visible_alias = "fdr")]
    FixtureDifficultyRating {
        #[arg(short, long)]
        team_id: Option<u64>,
        #[arg(short, long, default_value = "5")]
        limit: usize,
        #[arg(short, long)]
        all: bool,
    },
    /// Show gameweeks
    Gameweek {},
    /// Show manager's season history
    History(commands::HistoryArgs),
    /// Show live player stats for a specific event
    Live {
        event: u32,
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
        #[arg(long)]
        min_cost: Option<f64>,
        #[arg(long)]
        max_cost: Option<f64>,
        /// Show only available players
        #[arg(short, long)]
        available: bool,
    },
    /// Show a manager's team picks for a specific event
    Pick {
        /// Manager ID (entry ID)
        manager_id: u64,
        /// Event ID
        event_id: u32,
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
    /// Show league table
    Table {},
    /// Show top teams in the overall league
    Top {},
    /// Show teams
    Team {},
    /// Show team form based on total player form
    #[command(name = "team-form")]
    TeamForm {},
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
    /// Show popular transfers
    Transfer(commands::TransferArgs),
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
        Commands::Availability { team, all, limit } => {
            commands::handle_availability(team, all, limit).await?
        }
        Commands::Config(args) => commands::handle_config(args)?,
        Commands::DreamTeam { event_id } => commands::handle_dream_team(event_id).await?,
        Commands::Gameweek {} => commands::handle_gameweek().await?,
        Commands::History(args) => commands::handle_history(args).await?,
        Commands::Live { event, limit } => commands::handle_live(event, limit).await?,
        Commands::Manager { manager_id, gw } => commands::handle_manager(manager_id, gw).await?,
        Commands::MyTeam(args) => commands::handle_my_team(args).await?,
        Commands::Player {
            sort,
            position,
            limit,
            team,
            name,
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
                min_cost,
                max_cost,
                available,
            })
            .await?
        }
        Commands::Pick {
            manager_id,
            event_id,
        } => commands::handle_pick(manager_id, event_id).await?,
        Commands::PlayerSummary { player_id, graph } => {
            commands::handle_player_summary(player_id, graph).await?
        }
        Commands::SetPiece { team } => commands::handle_set_piece(team).await?,
        Commands::Status {} => commands::handle_status().await?,
        Commands::Table {} => commands::handle_table().await?,
        Commands::Top {} => commands::handle_top().await?,
        Commands::Team {} => commands::handle_team().await?,
        Commands::TeamForm {} => commands::handle_team_form().await?,
        Commands::TeamPerf { gw, last } => commands::handle_team_perf(gw, last).await?,
        Commands::Fixture(args) => commands::handle_fixture(args).await?,
        Commands::FixtureDifficultyRating {
            team_id,
            limit,
            all,
        } => commands::handle_fixture_difficulty_rating(team_id, limit, all).await?,
        Commands::Transfer(args) => commands::handle_transfer(args).await?,
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
