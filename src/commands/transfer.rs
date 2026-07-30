use crate::api::FplClient;
use crate::error::Result;
use crate::models::Position;
use crate::utils::constants::*;
use crate::utils::event_helpers::get_current_event_id;
use crate::utils::formatters::format_compact_number;
use crate::utils::team_helpers::create_team_short_name_map;
use clap::Args;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct TransferArgs {
    /// Show transfers OUT instead of IN
    #[arg(long, conflicts_with = "in_flag")]
    out: bool,

    /// Show transfers IN (default)
    #[arg(long = "in", conflicts_with = "out")]
    in_flag: bool,

    /// Number of players to show
    #[arg(short, long, default_value = "15")]
    limit: usize,

    /// Show all-time transfers instead of this gameweek
    #[arg(long)]
    all_time: bool,
}

pub async fn handle_transfer(args: TransferArgs) -> Result<()> {
    let bootstrap = FplClient::fetch_bootstrap_static().await?;

    // Build team name map using helper
    let team_map = create_team_short_name_map(&bootstrap.teams);

    // Find current gameweek using helper
    let current_gw = get_current_event_id(&bootstrap.events).unwrap_or(1);

    // Sort players by transfer direction (default is IN)
    let mut players = bootstrap.elements;
    let show_out = args.out;

    if show_out {
        if args.all_time {
            players.sort_by(|a, b| b.transfers_out.cmp(&a.transfers_out));
        } else {
            players.sort_by(|a, b| b.transfers_out_event.cmp(&a.transfers_out_event));
        }
    } else if args.all_time {
        players.sort_by(|a, b| b.transfers_in.cmp(&a.transfers_in));
    } else {
        players.sort_by(|a, b| b.transfers_in_event.cmp(&a.transfers_in_event));
    }

    // Print header
    let direction_str = if show_out { "OUT" } else { "IN" };

    let time_str = if args.all_time {
        "All-Time".to_string()
    } else {
        format!("GW{}", current_gw)
    };

    println!("\n=== Top Transfers {} ({}) ===\n", direction_str, time_str);

    println!(
        "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>trans_w$}  {:>trans_w$}  {:>trans_w$}",
        "Rank",
        "Name",
        "Pos",
        "Team",
        "Cost",
        "IN",
        "OUT",
        "Net",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        pos_w = WIDTH_POS,
        team_w = WIDTH_TEAM_SHORT_NAME,
        cost_w = WIDTH_COST,
        trans_w = WIDTH_TRANS,
    );

    for (i, player) in players.iter().take(args.limit).enumerate() {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("???");

        let position = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("???");

        let cost = format!("{:.1}", player.now_cost as f64 / 10.0);

        let (transfers_in, transfers_out) = if args.all_time {
            (player.transfers_in, player.transfers_out)
        } else {
            (player.transfers_in_event, player.transfers_out_event)
        };

        let net = transfers_in as i64 - transfers_out as i64;

        let in_str = format_compact_number(transfers_in);
        let out_str = format_compact_number(transfers_out);
        let net_str = if net >= 0 {
            format!("+{}", format_compact_number(net.unsigned_abs()))
        } else {
            format!("-{}", format_compact_number(net.unsigned_abs()))
        };

        // Apply padding first, then color
        let net_padded = format!("{:>width$}", net_str, width = WIDTH_TRANS);
        let net_colored = if net > 0 {
            net_padded.green().to_string()
        } else if net < 0 {
            net_padded.red().to_string()
        } else {
            net_padded
        };

        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>trans_w$}  {:>trans_w$}  {}",
            i + 1,
            player.web_name,
            position,
            team_name,
            cost,
            in_str,
            out_str,
            net_colored,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            cost_w = WIDTH_COST,
            trans_w = WIDTH_TRANS,
        );
    }

    println!();
    Ok(())
}
