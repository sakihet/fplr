mod api;
mod commands;
mod models;

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
    /// Show dream team
    DreamTeam { event_id: u32 },
    /// Show upcoming fixtures
    Fixture {},
    /// Show gameweeks
    Gameweek {},
    /// Show live player stats for a specific event
    Live {
        event: u32,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
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
    PlayerSummary { player_id: u64 },
    /// Show teams
    Team {},
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::DreamTeam { event_id } => commands::handle_dream_team(event_id).await,
        Commands::Gameweek {} => commands::handle_gameweek().await,
        Commands::Live { event, limit } => commands::handle_live(event, limit).await,
        Commands::Player {
            sort,
            position,
            limit,
            team,
        } => commands::handle_player(sort, position, limit, team).await,
        Commands::Pick {
            manager_id,
            event_id,
        } => commands::handle_pick(manager_id, event_id).await,
        Commands::PlayerSummary { player_id } => commands::handle_player_summary(player_id).await,
        Commands::Team {} => commands::handle_team().await,
        Commands::Fixture {} => commands::handle_fixture().await,
    }
}
