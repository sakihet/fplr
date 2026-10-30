mod api;
mod cache;
mod cli;
mod commands;
mod config;
mod error;
mod models;
mod utils;

use crate::error::{FplrError, Result};
use crate::models::{Position, SortBy};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
    /// Bypass the HTTP response cache
    #[arg(long, global = true)]
    no_cache: bool,
    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show replacement candidates for my team (position, budget, club limit)
    Buy(commands::BuyArgs),
    /// Manage the HTTP response cache
    Cache(commands::CacheArgs),
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },
    /// Manage configuration
    Config(commands::ConfigArgs),
    /// Show low-ownership players with high potential
    Differential {
        /// Maximum ownership threshold (%)
        #[arg(long, default_value = "10.0")]
        max_sel: f64,
        #[arg(short, long, default_value = "points")]
        sort: SortBy,
        #[arg(short, long)]
        position: Option<Position>,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Show dream team
    DreamTeam {
        /// Specific Gameweek (defaults to current)
        #[arg(short, long)]
        gw: Option<u32>,
    },
    /// Show fixtures, difficulty ratings and results
    Fixture(cli::FixtureArgs),
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
    /// Show mini-league standings
    #[command(name = "mini-league")]
    MiniLeague(commands::MiniLeagueArgs),
    /// Show the most valuable teams (squad value including bank)
    #[command(name = "most-valuable")]
    MostValuable {},
    /// Show my leagues
    #[command(name = "my-leagues")]
    MyLeagues(commands::MyLeaguesArgs),
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
    /// Show player lists, stats and summaries
    Player(cli::PlayerArgs),
    /// Show official price change progress
    #[command(name = "price-change")]
    PriceChange(commands::PriceChangeArgs),
    /// Show sell candidates from my team (availability, fixtures, minutes)
    Sell(commands::SellArgs),
    /// Show status
    Status {},
    /// Show league table
    Table {
        /// Include matches currently in play
        #[arg(short, long)]
        live: bool,
    },
    /// Show team stats, form and trends
    Team(cli::TeamArgs),
    /// Show template squad (top players by ownership per position)
    Template {},
    /// Show top teams in the overall league
    Top {},
    /// Show popular transfers
    Transfer(commands::TransferArgs),
    /// Show manager's transfer history
    #[command(name = "transfer-history")]
    TransferHistory(commands::TransferHistoryArgs),
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
    cache::set_no_cache(args.no_cache);
    utils::color::init(args.no_color);

    match args.commands {
        Commands::Buy(args) => commands::handle_buy(args).await?,
        Commands::Cache(args) => commands::handle_cache(args)?,
        Commands::Completions { shell } => {
            generate(shell, &mut Args::command(), "fplr", &mut std::io::stdout());
        }
        Commands::Config(args) => commands::handle_config(args)?,
        Commands::Differential {
            max_sel,
            sort,
            position,
            limit,
        } => commands::handle_differential(max_sel, sort, position, limit).await?,
        Commands::DreamTeam { gw } => commands::handle_dream_team(gw).await?,
        Commands::Fixture(args) => cli::fixture::run(args).await?,
        Commands::Gameweek {} => commands::handle_gameweek().await?,
        Commands::History(args) => commands::handle_history(args).await?,
        Commands::Live { gw, limit } => commands::handle_live(gw, limit).await?,
        Commands::Manager { manager_id, gw } => commands::handle_manager(manager_id, gw).await?,
        Commands::MiniLeague(args) => commands::handle_mini_league(args).await?,
        Commands::MostValuable {} => commands::handle_most_valuable().await?,
        Commands::MyLeagues(args) => commands::handle_my_leagues(args).await?,
        Commands::MyTeam(args) => commands::handle_my_team(args).await?,
        Commands::Pick {
            manager_id,
            event_id,
        } => commands::handle_pick(manager_id, event_id).await?,
        Commands::Player(args) => cli::player::run(args).await?,
        Commands::PriceChange(args) => commands::handle_price_change(args).await?,
        Commands::Sell(args) => commands::handle_sell(args).await?,
        Commands::Status {} => commands::handle_status().await?,
        Commands::Table { live } => commands::handle_table(live).await?,
        Commands::Team(args) => cli::team::run(args).await?,
        Commands::Template {} => commands::handle_template().await?,
        Commands::Top {} => commands::handle_top().await?,
        Commands::Transfer(args) => commands::handle_transfer(args).await?,
        Commands::TransferHistory(args) => commands::handle_transfer_history(args).await?,
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
