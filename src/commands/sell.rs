use crate::api::FplClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::{Element, LiveData, PlayerStatus, Position};
use crate::utils::color::Colorize;
use crate::utils::constants::*;
use crate::utils::event_helpers::get_current_event_id;
use crate::utils::fixture_helpers::{
    average_difficulty, format_upcoming_fixtures, upcoming_fixtures, upcoming_window, window_label,
};
use crate::utils::formatters::format_chance_of_playing;
use crate::utils::manager_helpers::fetch_picks_with_fallback;
use crate::utils::team_helpers::create_team_short_name_map;
use clap::Args;
use futures::future::join_all;
use std::collections::HashMap;

/// Average fixture difficulty at or above this is flagged as a tough run
const TOUGH_FDR_THRESHOLD: f64 = 3.5;
/// Average minutes per team match below this is flagged as a rotation risk
const LOW_MINUTES_THRESHOLD: f64 = 60.0;

#[derive(Debug, Args)]
pub struct SellArgs {
    /// Number of upcoming gameweeks to check fixtures for
    #[arg(short, long, default_value = "3")]
    weeks: u64,

    /// Number of recent finished gameweeks to check minutes for
    #[arg(short, long, default_value = "3")]
    last: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Flag {
    Injured,
    Suspended,
    Doubtful,
    Out,
    ToughFixtures,
    Blank,
    LowMinutes,
}

impl Flag {
    fn label(&self) -> &'static str {
        match self {
            Self::Blank => "BGW",
            Self::Doubtful => "DBT",
            Self::Injured => "INJ",
            Self::LowMinutes => "MIN",
            Self::Out => "OUT",
            Self::Suspended => "SUS",
            Self::ToughFixtures => "FIX",
        }
    }

    fn weight(&self) -> u32 {
        match self {
            Self::Injured | Self::Out | Self::Suspended => 3,
            Self::Doubtful => 2,
            Self::Blank | Self::LowMinutes | Self::ToughFixtures => 1,
        }
    }
}

struct Candidate<'a> {
    player: &'a Element,
    flags: Vec<Flag>,
    score: u32,
    minutes: Option<(u64, u64)>,      // (played, available)
    fixtures: Vec<Vec<(String, u8)>>, // per gameweek: (label, difficulty)
    avg_fdr: Option<f64>,
}

pub async fn handle_sell(args: SellArgs) -> Result<()> {
    let config = Config::load()?;
    let manager_id = config.get_manager_id()?;

    let bootstrap = FplClient::fetch_bootstrap_static().await?;
    let Some(event_id) = get_current_event_id(&bootstrap.events) else {
        println!("Season has not started yet.");
        return Ok(());
    };

    let picks_data = fetch_picks_with_fallback(manager_id, event_id).await?;
    let squad_event = picks_data.entry_history.event;

    let fixtures = FplClient::fetch_fixtures().await?;

    let window = upcoming_window(&bootstrap.events, args.weeks);

    // Minutes are taken from the most recent finished gameweeks only
    let mut recent_gws: Vec<u64> = bootstrap
        .events
        .iter()
        .filter(|e| e.finished)
        .map(|e| e.id)
        .collect();
    recent_gws.sort_unstable();
    let recent_gws: Vec<u64> = recent_gws.into_iter().rev().take(args.last).rev().collect();

    let live_results = join_all(
        recent_gws
            .iter()
            .map(|&gw| FplClient::fetch_live(gw as u32)),
    )
    .await;
    let mut live_histories: Vec<LiveData> = Vec::new();
    for res in live_results {
        live_histories.push(res?);
    }

    let mut minutes_map: HashMap<u64, u64> = HashMap::new();
    for live in &live_histories {
        for element in &live.elements {
            *minutes_map.entry(element.id).or_default() += element.stats.minutes;
        }
    }

    let team_map = create_team_short_name_map(&bootstrap.teams);
    let player_map: HashMap<u64, &Element> = bootstrap.elements.iter().map(|p| (p.id, p)).collect();

    let mut candidates: Vec<Candidate> = picks_data
        .picks
        .iter()
        .filter_map(|pick| player_map.get(&pick.element).copied())
        .map(|player| {
            let team_matches = fixtures
                .iter()
                .filter(|f| {
                    f.finished
                        && f.event.is_some_and(|e| recent_gws.contains(&e))
                        && (f.team_h == player.team || f.team_a == player.team)
                })
                .count() as u64;
            let minutes = (team_matches > 0).then(|| {
                (
                    minutes_map.get(&player.id).copied().unwrap_or(0),
                    team_matches * 90,
                )
            });

            let upcoming = upcoming_fixtures(&fixtures, player.team, &window, &team_map);
            let avg_fdr = average_difficulty(&upcoming);

            let mut flags = Vec::new();
            if let Some(flag) =
                availability_flag(&player.status, player.chance_of_playing_next_round)
            {
                flags.push(flag);
            }
            if avg_fdr.is_some_and(|avg| avg >= TOUGH_FDR_THRESHOLD) {
                flags.push(Flag::ToughFixtures);
            }
            if upcoming.iter().any(|gw| gw.is_empty()) {
                flags.push(Flag::Blank);
            }
            if minutes.is_some_and(|(played, _)| is_low_minutes(played, team_matches)) {
                flags.push(Flag::LowMinutes);
            }
            let score = flags.iter().map(Flag::weight).sum();

            Candidate {
                player,
                flags,
                score,
                minutes,
                fixtures: upcoming,
                avg_fdr,
            }
        })
        .collect();

    // Most urgent first; ties go to the player in worse form
    candidates.sort_by(|a, b| {
        b.score.cmp(&a.score).then_with(|| {
            let a_form: f64 = a.player.form.parse().unwrap_or(0.0);
            let b_form: f64 = b.player.form.parse().unwrap_or(0.0);
            a_form.total_cmp(&b_form)
        })
    });

    println!("\n=== Sell Candidates ===\n");
    println!(
        "Squad: GW{} | Fixtures: {} | Minutes: last {} finished GWs",
        squad_event,
        window_label(&window),
        recent_gws.len()
    );
    println!(
        "Flags: INJ/SUS/OUT/DBT availability, FIX avg FDR >= {:.1}, BGW blank, MIN < {:.0} mins/match\n",
        TOUGH_FDR_THRESHOLD, LOW_MINUTES_THRESHOLD
    );

    println!(
        "{:>id_w$}  {:<pos_w$}  {:<name_w$}  {:<team_w$}  {:>cost_w$}  {:>form_w$}  {:>avail_w$}  {:>min_w$}  {:>avg_w$}  {:<flags_w$}  Fixtures",
        "ID",
        "Pos",
        "Name",
        "Team",
        "Cost",
        "Form",
        "Avail",
        "Mins",
        "FDR",
        "Flags",
        id_w = WIDTH_ID,
        pos_w = WIDTH_POS,
        name_w = WIDTH_NAME,
        team_w = WIDTH_TEAM_SHORT_NAME,
        cost_w = WIDTH_COST,
        form_w = WIDTH_FORM,
        avail_w = WIDTH_AVAIL,
        min_w = WIDTH_MINUTES_RATIO,
        avg_w = WIDTH_AVG,
        flags_w = WIDTH_FLAGS,
    );

    for c in &candidates {
        let player = c.player;
        let pos_name = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("???");
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("???");
        let minutes = c
            .minutes
            .map(|(played, available)| format!("{}/{}", played, available))
            .unwrap_or_else(|| "-".to_string());
        let avg_fdr = c
            .avg_fdr
            .map(|avg| format!("{:.1}", avg))
            .unwrap_or_else(|| "-".to_string());

        let flags_plain = if c.flags.is_empty() {
            "-".to_string()
        } else {
            c.flags
                .iter()
                .map(Flag::label)
                .collect::<Vec<_>>()
                .join(" ")
        };
        let flags_padded = format!("{:<width$}", flags_plain, width = WIDTH_FLAGS);
        let flags_display = match c.score {
            0 => flags_padded.dimmed().to_string(),
            1 => flags_padded.yellow().to_string(),
            _ => flags_padded.red().bold().to_string(),
        };

        println!(
            "{:>id_w$}  {:<pos_w$}  {:<name_w$}  {:<team_w$}  {:>cost_w$.1}  {:>form_w$}  {}  {:>min_w$}  {:>avg_w$}  {}  {}",
            player.id,
            pos_name,
            player.web_name,
            team_name,
            player.now_cost as f64 / 10.0,
            player.form,
            format_chance_of_playing(player.chance_of_playing_next_round, &player.news),
            minutes,
            avg_fdr,
            flags_display,
            format_upcoming_fixtures(&c.fixtures),
            id_w = WIDTH_ID,
            pos_w = WIDTH_POS,
            name_w = WIDTH_NAME,
            team_w = WIDTH_TEAM_SHORT_NAME,
            cost_w = WIDTH_COST,
            form_w = WIDTH_FORM,
            min_w = WIDTH_MINUTES_RATIO,
            avg_w = WIDTH_AVG,
        );
    }

    if squad_event < event_id as u64 {
        println!(
            "\nNote: GW{} squad is not public yet, showing GW{} instead.",
            event_id, squad_event
        );
    }
    println!("\nNote: transfers made before the next deadline are not reflected.");
    println!("Tip: run `fplr buy --out <ID>` to find replacements.");

    Ok(())
}

fn availability_flag(status: &PlayerStatus, chance: Option<u64>) -> Option<Flag> {
    match status {
        PlayerStatus::Available => None,
        PlayerStatus::Doubtful => match chance {
            Some(c) if c >= 50 => Some(Flag::Doubtful),
            _ => Some(Flag::Out),
        },
        PlayerStatus::Injured => Some(Flag::Injured),
        PlayerStatus::Suspended => Some(Flag::Suspended),
        PlayerStatus::NotAvailable | PlayerStatus::Unavailable | PlayerStatus::Unknown => {
            Some(Flag::Out)
        }
    }
}

fn is_low_minutes(played: u64, matches: u64) -> bool {
    matches > 0 && (played as f64 / matches as f64) < LOW_MINUTES_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_player_has_no_flag() {
        assert_eq!(availability_flag(&PlayerStatus::Available, None), None);
    }

    #[test]
    fn doubtful_flag_depends_on_chance() {
        assert_eq!(
            availability_flag(&PlayerStatus::Doubtful, Some(75)),
            Some(Flag::Doubtful)
        );
        assert_eq!(
            availability_flag(&PlayerStatus::Doubtful, Some(25)),
            Some(Flag::Out)
        );
    }

    #[test]
    fn low_minutes_uses_per_match_average() {
        assert!(is_low_minutes(100, 2));
        assert!(!is_low_minutes(150, 2));
        assert!(!is_low_minutes(0, 0));
    }
}
