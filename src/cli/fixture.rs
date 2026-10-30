use crate::commands;
use crate::error::Result;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct FixtureArgs {
    #[command(subcommand)]
    command: FixtureCommands,
}

#[derive(Subcommand, Debug)]
enum FixtureCommands {
    /// Show fixture difficulty rating
    Fdr {
        #[arg(short, long)]
        team: Option<String>,
        #[arg(short, long, default_value = "5")]
        limit: usize,
        /// Start from this gameweek
        #[arg(short, long)]
        from: Option<u64>,
        /// Adjust difficulty by current team form
        #[arg(long)]
        form: bool,
        /// Sort teams by average difficulty (ascending)
        #[arg(long, conflicts_with = "form")]
        sort_by_avg: bool,
    },
    /// Show upcoming fixtures
    List(commands::FixtureArgs),
    /// Show season results matrix
    Results {},
    /// Show detailed points summary for a specific fixture
    Show {
        /// Fixture ID
        id: u64,
    },
    /// Show mathematically calculated Fixture Swings
    Swing(commands::SwingArgs),
}

pub async fn run(args: FixtureArgs) -> Result<()> {
    match args.command {
        FixtureCommands::Fdr {
            team,
            limit,
            from,
            form,
            sort_by_avg,
        } => {
            if form {
                commands::handle_fdr_form(team, limit, from).await?
            } else {
                commands::handle_fdr(team, limit, from, sort_by_avg).await?
            }
        }
        FixtureCommands::List(args) => commands::handle_fixture(args).await?,
        FixtureCommands::Results {} => commands::handle_results().await?,
        FixtureCommands::Show { id } => commands::handle_fixture_summary(id).await?,
        FixtureCommands::Swing(args) => commands::handle_swing(args).await?,
    }
    Ok(())
}
