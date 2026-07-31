use crate::api::FplClient;
use crate::error::Result;
use crate::utils::constants::{WIDTH_ENTRY_ID, WIDTH_LONG_NAME, WIDTH_RANK_WIDE, WIDTH_STAT};
use crate::utils::formatters::truncate;

pub async fn handle_top() -> Result<()> {
    // League 314 is the Overall league
    let standings = FplClient::fetch_league_standings(314).await?;

    println!("League: {}", standings.league.name);
    println!(
        "{:<rank_w$} {:<last_w$} {:<id_w$} {:<name_w$} {:<name_w$} {:<last_w$} {:<last_w$}",
        "Rank",
        "Last",
        "Manager ID",
        "Team Name",
        "Manager",
        "GW Pts",
        "Total Pts",
        rank_w = WIDTH_STAT,
        last_w = WIDTH_RANK_WIDE,
        id_w = WIDTH_ENTRY_ID,
        name_w = WIDTH_LONG_NAME,
    );

    for result in standings.standings.results {
        let last_rank = if result.last_rank == 0 {
            "-".to_string()
        } else {
            result.last_rank.to_string()
        };

        println!(
            "{:<rank_w$} {:<last_w$} {:<id_w$} {:<name_w$} {:<name_w$} {:<last_w$} {:<last_w$}",
            result.rank,
            last_rank,
            result.entry,
            truncate(&result.entry_name, WIDTH_LONG_NAME - 1),
            truncate(&result.player_name, WIDTH_LONG_NAME - 1),
            result.event_total,
            result.total,
            rank_w = WIDTH_STAT,
            last_w = WIDTH_RANK_WIDE,
            id_w = WIDTH_ENTRY_ID,
            name_w = WIDTH_LONG_NAME,
        );
    }

    Ok(())
}
