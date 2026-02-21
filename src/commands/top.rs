use crate::api::FplClient;
use crate::error::Result;
use crate::utils::formatters::truncate;

pub async fn handle_top() -> Result<()> {
    // League 314 is the Overall league
    let standings = FplClient::fetch_league_standings(314).await?;

    println!("League: {}", standings.league.name);
    println!(
        "{:<6} {:<10} {:<12} {:<25} {:<25} {:<10} {:<10}",
        "Rank", "Last", "Manager ID", "Team Name", "Manager", "GW Pts", "Total Pts"
    );

    for result in standings.standings.results {
        let last_rank = if result.last_rank == 0 {
            "-".to_string()
        } else {
            result.last_rank.to_string()
        };

        println!(
            "{:<6} {:<10} {:<12} {:<25} {:<25} {:<10} {:<10}",
            result.rank,
            last_rank,
            result.entry,
            truncate(&result.entry_name, 24),
            truncate(&result.player_name, 24),
            result.event_total,
            result.total
        );
    }

    Ok(())
}
