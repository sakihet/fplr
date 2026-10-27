use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::*;
use crate::utils::formatters::truncate;

pub async fn handle_most_valuable() -> Result<()> {
    let teams = FplClient::fetch_most_valuable_teams().await?;

    println!("\n=== Most Valuable Teams ===\n");

    println!(
        "{:>rank_w$}  {:>id_w$}  {:<name_w$}  {:<name_w$}  {:>value_w$}  {:>transfers_w$}",
        "Rank",
        "Manager ID",
        "Team Name",
        "Manager",
        "Value",
        "Transfers",
        rank_w = WIDTH_RANK,
        id_w = WIDTH_ENTRY_ID,
        name_w = WIDTH_LONG_NAME,
        value_w = WIDTH_STAT,
        transfers_w = WIDTH_TRANSFERS,
    );

    for (i, team) in teams.iter().enumerate() {
        // Value is in tenths of a million, including money in the bank
        let value = format!("{:.1}", team.value_with_bank as f64 / 10.0);

        println!(
            "{:>rank_w$}  {:>id_w$}  {:<name_w$}  {:<name_w$}  {:>value_w$}  {:>transfers_w$}",
            i + 1,
            team.entry,
            truncate(&team.name, WIDTH_LONG_NAME - 1),
            truncate(&team.player_name, WIDTH_LONG_NAME - 1),
            value,
            team.total_transfers,
            rank_w = WIDTH_RANK,
            id_w = WIDTH_ENTRY_ID,
            name_w = WIDTH_LONG_NAME,
            value_w = WIDTH_STAT,
            transfers_w = WIDTH_TRANSFERS,
        );
    }

    println!();
    Ok(())
}
