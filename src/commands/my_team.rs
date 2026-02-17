use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::{Pick, Position};
use crate::utils::event_helpers::get_effective_event_id;
use crate::utils::formatters::format_chance_of_playing;
use crate::utils::team_helpers::create_team_short_name_map;
use clap::Args;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Args)]
pub struct MyTeamArgs {
    /// Optional Gameweek ID. If not provided, uses the current Gameweek.
    #[arg(short, long)]
    gw: Option<u32>,
}

pub async fn handle_my_team(args: MyTeamArgs) -> Result<()> {
    // 1. Load Config and get manager ID
    let config = Config::load()?;
    let manager_id = config.get_manager_id()?;

    // 2. Fetch Bootstrap Static to get current GW and player details
    let bootstrap = FplClient::fetch_bootstrap_static().await?;

    // Determine Gameweek using helper
    let event_id = match get_effective_event_id(&bootstrap.events, args.gw) {
        Some(id) => id,
        None => {
            println!("Could not determine current Gameweek.");
            return Ok(());
        }
    };

    println!(
        "Fetching team for Manager ID: {} (GW {})",
        manager_id, event_id
    );

    // 3. Fetch Manager Picks
    let picks_data = FplClient::fetch_manager_picks(manager_id, event_id).await?;

    // 4. Fetch Live Data for points
    let live_data = FplClient::fetch_live(event_id).await?;

    // 5. Fetch Fixtures to check if matches have started and get opponents
    let fixtures = FplClient::fetch_fixtures_by_event(event_id).await?;
    let mut started_teams = HashSet::new();
    let short_team_map = create_team_short_name_map(&bootstrap.teams);
    let mut team_fixtures: HashMap<u64, Vec<String>> = HashMap::new();

    for fixture in &fixtures {
        if fixture.started.unwrap_or(false) {
            started_teams.insert(fixture.team_h);
            started_teams.insert(fixture.team_a);
        }

        let home_team_short = short_team_map
            .get(&fixture.team_h)
            .map(|s| s.as_str())
            .unwrap_or("???");
        let away_team_short = short_team_map
            .get(&fixture.team_a)
            .map(|s| s.as_str())
            .unwrap_or("???");

        team_fixtures
            .entry(fixture.team_h)
            .or_default()
            .push(format!("{}(H)", away_team_short));
        team_fixtures
            .entry(fixture.team_a)
            .or_default()
            .push(format!("{}(A)", home_team_short));
    }

    // Helper maps
    let team_map = create_team_short_name_map(&bootstrap.teams);
    let player_map: HashMap<u64, &crate::models::Element> =
        bootstrap.elements.iter().map(|p| (p.id, p)).collect();

    let live_map: HashMap<u64, &crate::models::LiveStats> = live_data
        .elements
        .iter()
        .map(|e| (e.id, &e.stats))
        .collect();

    // 6. Display
    // Sort picks by position: GKP(1), DEF(2), MID(3), FWD(4), then Sub
    // Picks 1-11 are starters, 12-15 are bench.

    let mut starters: Vec<&Pick> = picks_data
        .picks
        .iter()
        .filter(|p| p.position <= 11)
        .collect();
    let bench: Vec<&Pick> = picks_data
        .picks
        .iter()
        .filter(|p| p.position > 11)
        .collect();

    // Sort starters by position type
    starters.sort_by_key(|p| {
        if let Some(player) = player_map.get(&p.element) {
            player.element_type
        } else {
            99 // unknown
        }
    });

    // Header
    println!(
        "\n{:<15} GW{} Points: {}",
        "My Team", event_id, picks_data.entry_history.points
    );
    println!(
        "Overall Rank: {}",
        picks_data
            .entry_history
            .overall_rank
            .map(|r| r.to_string())
            .unwrap_or("N/A".to_string())
    );
    println!(
        "Bank:         £{:.1}m",
        picks_data.entry_history.bank as f64 / 10.0
    );
    println!();
    println!(
        "{:<4} {:<4} {:<20} {:<6} {:<15} {:>5}  {:<5} {:<6} {:<6} {:<32}",
        "ID", "Pos", "Name", "Team", "Opp", "Avail", "Pts", "Cost", "Form", "Status"
    );

    // Function to print a player row
    let print_player = |pick: &Pick, _is_bench: bool| {
        let player_opt = player_map.get(&pick.element);
        let live_opt = live_map.get(&pick.element);

        if let Some(player) = player_opt {
            let pos_name = Position::from_element_type_id(player.element_type)
                .map(|p| p.display_name())
                .unwrap_or("???");

            let team_name = team_map
                .get(&player.team)
                .map(|s| s.as_str())
                .unwrap_or("???");

            let opponents = team_fixtures
                .get(&player.team)
                .map(|ops| ops.join(","))
                .unwrap_or_else(|| "-".to_string());

            let avail_display =
                format_chance_of_playing(player.chance_of_playing_next_round, &player.news);

            let mut name_display = player.web_name.clone();
            if pick.is_captain {
                name_display.push_str(" (C)");
            }
            if pick.is_vice_captain {
                name_display.push_str(" (VC)");
            }

            let points = live_opt.map(|l| l.total_points).unwrap_or(0);
            // Apply multiplier for display (e.g. Captain points doubled)
            let final_points = points * (pick.multiplier as i64);

            let points_display = if !started_teams.contains(&player.team) && points == 0 {
                "-".to_string()
            } else {
                final_points.to_string()
            };

            let cost = format!("{:.1}", player.now_cost as f64 / 10.0);

            println!(
                "{:<4} {:<4} {:<20} {:<6} {:<15} {}  {:<5} {:<6} {:<6} {:<32}",
                player.id,
                pos_name,
                name_display,
                team_name,
                opponents,
                avail_display,
                points_display,
                cost,
                player.form,
                player.news.chars().take(32).collect::<String>()
            );
        } else {
            println!("Unknown Player ID: {}", pick.element);
        }
    };

    println!("Starters:");
    for pick in starters {
        print_player(pick, false);
    }

    println!("\nBench:");
    for pick in bench {
        print_player(pick, true);
    }

    Ok(())
}
