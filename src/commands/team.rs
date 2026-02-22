use crate::api::FplClient;
use crate::error::Result;
use crate::models::TeamSortBy;
use crate::utils::constants::*;

pub async fn handle_team(sort_by: &TeamSortBy) -> Result<()> {
    let mut data = FplClient::fetch_bootstrap_static().await?;

    match sort_by {
        TeamSortBy::Position => data.teams.sort_by(|a, b| a.position.cmp(&b.position)),
        TeamSortBy::Strength => data.teams.sort_by(|a, b| b.strength.cmp(&a.strength)),
    }

    println!(
        "{:<team_w$}  {:<name_w$}  {:>pos_w$}  {:>str_w$}",
        "Team",
        "Name",
        "Pos",
        "Str",
        team_w = WIDTH_TEAM_SHORT_NAME,
        name_w = WIDTH_TEAM_NAME,
        pos_w = WIDTH_POS,
        str_w = WIDTH_STR,
    );

    for team in data.teams {
        println!(
            "{:<team_w$}  {:<name_w$}  {:>pos_w$}  {:>str_w$}",
            team.short_name,
            team.name,
            team.position,
            team.strength,
            team_w = WIDTH_TEAM_SHORT_NAME,
            name_w = WIDTH_TEAM_NAME,
            pos_w = WIDTH_POS,
            str_w = WIDTH_STR,
        );
    }

    Ok(())
}
