use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::event_helpers::find_next_event;
use crate::utils::formatters::format_datetime_local;
use crate::utils::team_helpers::create_team_map;
use clap::Args;

#[derive(Debug, Args)]
pub struct FixtureArgs {
    /// Specific Gameweek (defaults to next)
    #[arg(short, long)]
    pub gw: Option<u32>,
}

pub async fn handle_fixture(args: FixtureArgs) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_map(&bootstrap_data.teams);

    let event_id = match args.gw {
        Some(gw) => gw as u64,
        None => {
            let next_event =
                find_next_event(&bootstrap_data.events).ok_or(FplrError::NoNextEvent)?;
            next_event.id
        }
    };

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
        "{:<4} {:<20} {:<20} {:<20} {:<10}",
        "ID", "Kickoff Time", "Home", "Away", "Score"
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
            "{:<4} {:<20} {:<20} {:<20} {:<10}",
            fixture.id,
            format_datetime_local(kickoff),
            home_team,
            away_team,
            score
        );
    }

    Ok(())
}
