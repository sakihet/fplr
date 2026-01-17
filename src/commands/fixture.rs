use crate::api::FplClient;
use crate::utils::formatters::format_datetime;
use crate::utils::team_helpers::create_team_map;

pub async fn handle_fixture() {
    let bootstrap_data = match FplClient::fetch_bootstrap_static().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching bootstrap data: {}", e);
            return;
        }
    };

    let team_map = create_team_map(&bootstrap_data.teams);

    let next_event = match bootstrap_data.events.iter().find(|e| e.is_next) {
        Some(event) => event,
        None => {
            eprintln!("No next event found");
            return;
        }
    };

    let fixtures = match FplClient::fetch_fixtures().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching fixtures: {}", e);
            return;
        }
    };

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
            format_datetime(kickoff),
            home_team,
            away_team
        );
    }
}
