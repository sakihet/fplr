use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::formatters::format_datetime_local;
use crate::utils::player_helpers::create_player_map;
use clap::Args;

#[derive(Debug, Args)]
pub struct TransferHistoryArgs {
    /// Manager ID (uses configured ID if not provided)
    #[arg(short, long)]
    manager_id: Option<u64>,
}

pub async fn handle_transfer_history(args: TransferHistoryArgs) -> Result<()> {
    let manager_id = if let Some(id) = args.manager_id {
        id
    } else {
        Config::load()?.get_manager_id()?
    };

    let (transfers, bootstrap) = tokio::join!(
        FplClient::fetch_manager_transfers(manager_id),
        FplClient::fetch_bootstrap_static()
    );

    let mut transfers = transfers?;
    let bootstrap = bootstrap?;

    if transfers.is_empty() {
        println!("No transfer history for Manager ID: {}", manager_id);
        return Ok(());
    }

    let player_map = create_player_map(&bootstrap.elements);

    transfers.sort_by_key(|t| t.event);

    println!(
        "{:<gw_w$}  {:<time_w$}  {:<name_w$}  {:>cost_w$}  {:<name_w$}  {:>cost_w$}",
        "GW",
        "Time",
        "Out",
        "Out£",
        "In",
        "In£",
        gw_w = WIDTH_GW,
        time_w = WIDTH_TIME,
        name_w = WIDTH_NAME,
        cost_w = WIDTH_COST,
    );

    for t in &transfers {
        let out_name = player_map
            .get(&t.element_out)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let in_name = player_map
            .get(&t.element_in)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let out_cost = format!("{:.1}", t.element_out_cost as f64 / 10.0);
        let in_cost = format!("{:.1}", t.element_in_cost as f64 / 10.0);
        let time_str = format_datetime_local(&t.time);

        println!(
            "{:<gw_w$}  {:<time_w$}  {:<name_w$}  {:>cost_w$}  {:<name_w$}  {:>cost_w$}",
            t.event,
            time_str,
            out_name,
            out_cost,
            in_name,
            in_cost,
            gw_w = WIDTH_GW,
            time_w = WIDTH_TIME,
            name_w = WIDTH_NAME,
            cost_w = WIDTH_COST,
        );
    }

    Ok(())
}
