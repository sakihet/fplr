use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::event_helpers::find_next_event;
use crate::utils::formatters::format_datetime_local;
use crate::utils::team_helpers::create_team_map;

pub async fn handle_fixture() -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_map(&bootstrap_data.teams);

    let next_event = find_next_event(&bootstrap_data.events).ok_or(FplrError::NoNextEvent)?;

    let fixtures = FplClient::fetch_fixtures().await?;

    let mut next_fixtures: Vec<_> = fixtures
        .iter()
        .filter(|f| f.event == Some(next_event.id) && !f.finished)
        .collect();

    next_fixtures.sort_by(|a, b| a.kickoff_time.cmp(&b.kickoff_time));

    println!(
        "{:<4} {:<20} {:<20} {:<20}",
        "ID", "Kickoff Time", "Home", "Away"
    );

    for fixture in next_fixtures {
        let home_team = team_map
            .get(&fixture.team_h)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let away_team = team_map
            .get(&fixture.team_a)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let kickoff = fixture.kickoff_time.as_deref().unwrap_or("");

        println!(
            "{:<4} {:<20} {:<20} {:<20}",
            fixture.id,
            format_datetime_local(kickoff),
            home_team,
            away_team
        );
    }

    Ok(())
}
