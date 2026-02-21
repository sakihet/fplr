use crate::api::FplClient;
use crate::error::Result;
use crate::utils::formatters::*;

pub async fn handle_team() -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    println!(
        "{:>id_w$}  {:<name_w$}  {:<team_w$}  {:>str_w$}",
        "ID",
        "Name",
        "Team",
        "Str",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        team_w = WIDTH_TEAM,
        str_w = WIDTH_STR,
    );

    for team in data.teams {
        println!(
            "{:>id_w$}  {:<name_w$}  {:<team_w$}  {:>str_w$}",
            team.id,
            truncate(&team.name, WIDTH_NAME),
            team.short_name,
            team.strength,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            team_w = WIDTH_TEAM,
            str_w = WIDTH_STR,
        );
    }

    Ok(())
}
