use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position};
use crate::utils::color::Colorize;
use crate::utils::constants::*;
use crate::utils::formatters::format_datetime_local;
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};
use chrono::{DateTime, Utc};
use clap::Args;

#[derive(Debug, Args)]
pub struct PriceChangeArgs {
    /// Show players closest to a price fall instead of a rise
    #[arg(long)]
    fall: bool,

    /// Filter by team name
    #[arg(short, long)]
    team: Option<String>,

    /// Filter by position
    #[arg(short, long)]
    position: Option<Position>,

    /// Number of players to show
    #[arg(short, long, default_value = "20")]
    limit: usize,
}

pub async fn handle_price_change(args: PriceChangeArgs) -> Result<()> {
    let bootstrap = FplClient::fetch_bootstrap_static().await?;

    let team_map = create_team_short_name_map(&bootstrap.teams);
    let target_team_ids = args
        .team
        .as_ref()
        .map(|name| find_team_ids_by_name(&bootstrap.teams, name));

    let mut players: Vec<&Element> = bootstrap
        .elements
        .iter()
        .filter(|p| p.price_change_percent.is_some())
        .filter(|p| {
            target_team_ids
                .as_ref()
                .is_none_or(|ids| ids.contains(&p.team))
        })
        .filter(|p| {
            args.position
                .as_ref()
                .is_none_or(|pos| p.element_type == pos.element_type_id() as u64)
        })
        .collect();

    sort_players(&mut players, args.fall);

    let direction_str = if args.fall { "Falls" } else { "Rises" };
    println!("\n=== Price Change Progress ({}) ===\n", direction_str);

    if let Some(updated) = &bootstrap.game_config.status.price_change_last_updated {
        println!("Last updated: {}\n", format_datetime_local(updated));
    }

    println!(
        "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>pct_w$}  {:>rate_w$}  Locked until",
        "Rank",
        "Name",
        "Pos",
        "Team",
        "Cost",
        "Now%",
        "Net/h",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pos_w = WIDTH_POS,
        team_w = WIDTH_TEAM_SHORT_NAME,
        cost_w = WIDTH_COST,
        pct_w = WIDTH_PCT,
        rate_w = WIDTH_RATE,
    );

    let now = Utc::now();

    for (i, player) in players.iter().take(args.limit).enumerate() {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("???");

        let position = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("???");

        let cost = format!("{:.1}", player.now_cost as f64 / 10.0);

        // Apply padding first, then color
        let pct = progress(player);
        let pct_padded = format!("{:>width$.1}", pct, width = WIDTH_PCT);
        let pct_colored = if pct > 0.0 {
            pct_padded.green().to_string()
        } else if pct < 0.0 {
            pct_padded.red().to_string()
        } else {
            pct_padded
        };

        let mut line = format!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {}  {:>+rate_w$}",
            i + 1,
            player.web_name,
            position,
            team_name,
            cost,
            pct_colored,
            player.price_change_hourly_rate,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            cost_w = WIDTH_COST,
            rate_w = WIDTH_RATE,
        );

        if let Some(locked) = locked_until(player, now) {
            line.push_str("  ");
            line.push_str(&format_datetime_local(locked).dimmed().to_string());
        }

        println!("{}", line);
    }

    println!();
    Ok(())
}

fn progress(player: &Element) -> f64 {
    player
        .price_change_percent
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0)
}

/// Returns the lock expiry only while the lock is still in effect.
fn locked_until(player: &Element, now: DateTime<Utc>) -> Option<&str> {
    let locked = player.price_change_locked_until.as_deref()?;
    let until = locked.parse::<DateTime<Utc>>().ok()?;
    (until > now).then_some(locked)
}

/// Sort by progress towards a rise (descending) or a fall (ascending), breaking
/// ties by name for stable output.
fn sort_players(players: &mut [&Element], fall: bool) {
    players.sort_by(|a, b| {
        let (pa, pb) = (progress(a), progress(b));
        let ord = if fall {
            pa.total_cmp(&pb)
        } else {
            pb.total_cmp(&pa)
        };
        ord.then_with(|| a.web_name.cmp(&b.web_name))
    });
}
