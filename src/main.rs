mod api;
mod commands;
mod config;
mod models;
mod utils;

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
    /// Manage configuration
    Config(commands::ConfigArgs),
    /// Show dream team
    DreamTeam { event_id: u32 },
    /// Show upcoming fixtures
    Fixture {},
    /// Show fixture difficulty rating
    #[command(alias = "fdr")]
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
    /// Show live player stats for a specific event
    Live {
        event: u32,
        #[arg(short, long, default_value = "20")]
        limit: usize,
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
    /// Show status
    Status {},
    /// Show league table
    Table {},
    /// Show teams
    Team {},
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Config(args) => commands::handle_config(args),
        Commands::DreamTeam { event_id } => commands::handle_dream_team(event_id).await,
        Commands::Gameweek {} => commands::handle_gameweek().await,
        Commands::Live { event, limit } => commands::handle_live(event, limit).await,
        Commands::MyTeam(args) => commands::handle_my_team(args).await,
        Commands::Player {
            sort,
            position,
            limit,
            team,
            name,
        } => commands::handle_player(sort, position, limit, team, name).await,
        Commands::Pick {
            manager_id,
            event_id,
        } => commands::handle_pick(manager_id, event_id).await,
        Commands::PlayerSummary { player_id, graph } => {
            commands::handle_player_summary(player_id, graph).await
        }
        Commands::Status {} => commands::handle_status().await,
        Commands::Table {} => commands::handle_table().await,
        Commands::Team {} => commands::handle_team().await,
        Commands::Fixture {} => commands::handle_fixture().await,
        Commands::FixtureDifficultyRating {
            team_id,
            limit,
            all,
        } => commands::handle_fixture_difficulty_rating(team_id, limit, all).await,
    }
}
