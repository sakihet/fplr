use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::event_helpers::get_effective_event_id;
use crate::utils::formatters::*;
use crate::utils::team_helpers::create_team_map;
use clap::Args;

#[derive(Debug, Args)]
pub struct FixtureArgs {
    /// Specific Gameweek (defaults to current)
    #[arg(short, long)]
    pub gw: Option<u32>,
}

pub async fn handle_fixture(args: FixtureArgs) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_map(&bootstrap_data.teams);

    let event_id = get_effective_event_id(&bootstrap_data.events, args.gw)
        .ok_or(FplrError::NoNextEvent)? as u64;

    let fixtures = FplClient::fetch_fixtures().await?;

    let mut target_fixtures: Vec<_> = fixtures
        .iter()
        .filter(|f| f.event == Some(event_id))
        .collect();

    if target_fixtures.is_empty() {
        println!("No fixtures found for Gameweek {}.", event_id);
        return Ok(());
    }

    target_fixtures.sort_by(|a, b| a.kickoff_time.cmp(&b.kickoff_time));

    println!("Fixtures for Gameweek {}:", event_id);
    println!(
        "{:>id_w$}  {:<time_w$}  {:<home_w$}  {:<away_w$}  {:<score_w$}",
        "ID",
        "Kickoff Time",
        "Home",
        "Away",
        "Score",
        id_w = WIDTH_ID,
        time_w = WIDTH_TIME,
        home_w = WIDTH_NAME,
        away_w = WIDTH_NAME,
        score_w = WIDTH_SCORE,
    );

    for fixture in target_fixtures {
        let home_team = team_map
            .get(&fixture.team_h)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let away_team = team_map
            .get(&fixture.team_a)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let kickoff = fixture.kickoff_time.as_deref().unwrap_or("");

        let score = if fixture.finished {
            format!(
                "{} - {}",
                fixture.team_h_score.unwrap_or(0),
                fixture.team_a_score.unwrap_or(0)
            )
        } else {
            "-".to_string()
        };

        println!(
            "{:>id_w$}  {:<time_w$}  {:<home_w$}  {:<away_w$}  {:<score_w$}",
            fixture.id,
            format_datetime_local(kickoff),
            truncate(home_team, WIDTH_NAME),
            truncate(away_team, WIDTH_NAME),
            score,
            id_w = WIDTH_ID,
            time_w = WIDTH_TIME,
            home_w = WIDTH_NAME,
            away_w = WIDTH_NAME,
            score_w = WIDTH_SCORE,
        );
    }

    Ok(())
}
