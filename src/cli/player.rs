use crate::commands;
use crate::error::Result;
use crate::models::{Position, SetPieceType, SortBy, XaSortBy, XgSortBy, XgcSortBy, XgiSortBy};
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct PlayerArgs {
    #[command(subcommand)]
    command: PlayerCommands,
}

#[derive(Subcommand, Debug)]
enum PlayerCommands {
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
    /// Show a player's performance in previous seasons
    History {
        /// Player ID
        player_id: u64,
    },
    /// Show players
    List {
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
        region: Option<String>,
        #[arg(long)]
        min_cost: Option<f64>,
        #[arg(long)]
        max_cost: Option<f64>,
        /// Show only available players
        #[arg(short, long)]
        available: bool,
    },
    /// Show regions, or players from a specific region if a name is given
    Region {
        /// Region name (shows players from this region instead of the region list)
        name: Option<String>,
    },
    /// Show set piece takers (penalties, free kicks, corners)
    SetPiece {
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by set piece type
        #[arg(long = "type", value_name = "TYPE")]
        kind: Option<SetPieceType>,
    },
    /// Show player summary
    Show {
        player_id: u64,
        #[arg(short, long)]
        graph: bool,
        /// Show xG/xA/xGI/xGC stats
        #[arg(long)]
        xg: bool,
        /// Show ICT index stats
        #[arg(long)]
        ict: bool,
        /// Show FPL management stats (price, ownership, transfers)
        #[arg(long)]
        fpl: bool,
    },
    /// Show talisman players
    Talisman {
        /// Filter by team name
        #[arg(short, long)]
        team: Option<String>,
    },
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
        sort: XaSortBy,
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
        sort: XgSortBy,
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
        sort: XgcSortBy,
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
        sort: XgiSortBy,
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

pub async fn run(args: PlayerArgs) -> Result<()> {
    match args.command {
        PlayerCommands::Availability {
            team,
            name,
            news,
            position,
            all,
            limit,
        } => commands::handle_availability(team, name, news, position, all, limit).await?,
        PlayerCommands::Compare { id1, id2 } => commands::handle_compare(id1, id2).await?,
        PlayerCommands::History { player_id } => commands::handle_history_past(player_id).await?,
        PlayerCommands::List {
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
                max_sel: None,
            })
            .await?
        }
        PlayerCommands::Region { name } => match name {
            Some(name) => {
                commands::handle_player(commands::PlayerFilterArgs {
                    region: Some(name),
                    limit: 20,
                    ..Default::default()
                })
                .await?
            }
            None => commands::handle_region().await?,
        },
        PlayerCommands::SetPiece { team, kind } => commands::handle_set_piece(team, kind).await?,
        PlayerCommands::Show {
            player_id,
            graph,
            xg,
            ict,
            fpl,
        } => commands::handle_player_summary(player_id, graph, xg, ict, fpl).await?,
        PlayerCommands::Talisman { team } => commands::handle_talisman(team).await?,
        PlayerCommands::Trend {
            team,
            position,
            limit,
            weeks,
            min_cost,
            max_cost,
        } => commands::handle_trend(team, position, limit, weeks, min_cost, max_cost).await?,
        PlayerCommands::Xa {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xa(sort, team, position, limit).await?,
        PlayerCommands::Xg {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xg(sort, team, position, limit).await?,
        PlayerCommands::Xgc {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xgc(sort, team, position, limit).await?,
        PlayerCommands::Xgi {
            sort,
            team,
            position,
            limit,
        } => commands::handle_xgi(sort, team, position, limit).await?,
    }
    Ok(())
}
