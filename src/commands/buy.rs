use crate::api::FplClient;
use crate::config::Config;
use crate::error::{FplrError, Result};
use crate::models::{BuySortBy, Element, Position};
use crate::utils::color::Colorize;
use crate::utils::constants::*;
use crate::utils::event_helpers::get_current_event_id;
use crate::utils::fixture_helpers::{
    average_difficulty, format_upcoming_fixtures, upcoming_fixtures, upcoming_window, window_label,
};
use crate::utils::manager_helpers::fetch_picks_with_fallback;
use crate::utils::team_helpers::create_team_short_name_map;
use clap::Args;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Players averaging fewer minutes per team match than this are excluded
const MIN_MINUTES_PER_MATCH: f64 = 60.0;
/// FPL allows at most this many players from one club
const MAX_PER_CLUB: usize = 3;
/// Average fixture difficulty at or below this is shown as an easy run
const EASY_FDR_THRESHOLD: f64 = 2.5;
/// Average fixture difficulty at or above this is shown as a tough run
const TOUGH_FDR_THRESHOLD: f64 = 3.5;

#[derive(Debug, Args)]
pub struct BuyArgs {
    /// Player ID to sell (repeatable); limits candidates to the same position and budget
    #[arg(short, long = "out", value_name = "ID")]
    out: Vec<u64>,

    /// Filter by position
    #[arg(short, long)]
    position: Option<Position>,

    /// Maximum cost (e.g. 7.5)
    #[arg(long)]
    max_cost: Option<f64>,

    /// Sort by metric
    #[arg(short, long, default_value = "form")]
    sort: BuySortBy,

    /// Number of upcoming gameweeks to check fixtures for
    #[arg(short, long, default_value = "3")]
    weeks: u64,

    /// Number of players to show per position
    #[arg(short, long, default_value = "10")]
    limit: usize,
}

struct Candidate<'a> {
    player: &'a Element,
    form: f64,
    xgi_per_90: f64,
    minutes_per_match: f64,
    fixtures: Vec<Vec<(String, u8)>>, // per gameweek: (label, difficulty)
    avg_fdr: Option<f64>,
}

pub async fn handle_buy(args: BuyArgs) -> Result<()> {
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

    let team_map = create_team_short_name_map(&bootstrap.teams);
    let player_map: HashMap<u64, &Element> = bootstrap.elements.iter().map(|p| (p.id, p)).collect();
    let owned: HashSet<u64> = picks_data.picks.iter().map(|p| p.element).collect();

    let mut outs: Vec<&Element> = Vec::new();
    for id in &args.out {
        if !owned.contains(id) {
            println!("Player {} is not in your GW{} squad.", id, squad_event);
            return Ok(());
        }
        outs.push(
            player_map
                .get(id)
                .copied()
                .ok_or(FplrError::PlayerNotFound(*id))?,
        );
    }

    // Selling price is not public, so the current price is used as an upper bound
    // Prices are compared in tenths of a million to avoid float rounding
    let bank = picks_data.entry_history.bank.max(0) as u64;
    let out_value: u64 = outs.iter().map(|p| p.now_cost).sum();
    let max_cost = args.max_cost.map(|c| (c * 10.0).round() as u64);
    let budget = match (outs.is_empty(), max_cost) {
        (true, max_cost) => max_cost,
        (false, Some(max_cost)) => Some((bank + out_value).min(max_cost)),
        (false, None) => Some(bank + out_value),
    };

    // Club counts after the outgoing players leave
    let mut club_counts: HashMap<u64, usize> = HashMap::new();
    for id in owned.iter().filter(|id| !args.out.contains(id)) {
        if let Some(player) = player_map.get(id) {
            *club_counts.entry(player.team).or_default() += 1;
        }
    }

    // One section per outgoing position, or a single section when browsing
    let sections: Vec<Option<u64>> = if outs.is_empty() {
        vec![args.position.as_ref().map(|p| p.element_type_id() as u64)]
    } else {
        let positions: BTreeSet<u64> = outs.iter().map(|p| p.element_type).collect();
        positions
            .into_iter()
            .filter(|pos| {
                args.position
                    .as_ref()
                    .is_none_or(|p| p.element_type_id() as u64 == *pos)
            })
            .map(Some)
            .collect()
    };

    let mut team_matches: HashMap<u64, u64> = HashMap::new();
    for f in fixtures.iter().filter(|f| f.finished) {
        *team_matches.entry(f.team_h).or_default() += 1;
        *team_matches.entry(f.team_a).or_default() += 1;
    }

    let window = upcoming_window(&bootstrap.events, args.weeks);

    let candidates: Vec<Candidate> = bootstrap
        .elements
        .iter()
        .filter(|p| !owned.contains(&p.id))
        .filter(|p| p.status.is_available(p.chance_of_playing_next_round))
        .filter(|p| budget.is_none_or(|b| p.now_cost <= b))
        .filter(|p| club_counts.get(&p.team).copied().unwrap_or(0) < MAX_PER_CLUB)
        .filter_map(|player| {
            let matches = team_matches.get(&player.team).copied().unwrap_or(0);
            if matches == 0 {
                return None;
            }
            let minutes_per_match = player.minutes as f64 / matches as f64;
            if minutes_per_match < MIN_MINUTES_PER_MATCH {
                return None;
            }

            let xgi: f64 = player.expected_goal_involvements.parse().unwrap_or(0.0);
            let xgi_per_90 = if player.minutes > 0 {
                xgi / player.minutes as f64 * 90.0
            } else {
                0.0
            };
            let upcoming = upcoming_fixtures(&fixtures, player.team, &window, &team_map);
            let avg_fdr = average_difficulty(&upcoming);

            Some(Candidate {
                player,
                form: player.form.parse().unwrap_or(0.0),
                xgi_per_90,
                minutes_per_match,
                fixtures: upcoming,
                avg_fdr,
            })
        })
        .collect();

    println!("\n=== Buy Candidates ===\n");
    print!(
        "Squad: GW{} | Bank: £{:.1}m | Fixtures: {}",
        squad_event,
        bank as f64 / 10.0,
        window_label(&window)
    );
    if let Some(b) = budget {
        print!(" | Budget: £{:.1}m", b as f64 / 10.0);
    }
    println!();
    if !outs.is_empty() {
        let names: Vec<String> = outs
            .iter()
            .map(|p| format!("{} (£{:.1}m)", p.web_name, p.now_cost as f64 / 10.0))
            .collect();
        println!("Out: {}", names.join(", "));
    }
    println!(
        "Mins: average per team match | Excluded: owned, unavailable, < {:.0} mins/match, clubs with {} players\n",
        MIN_MINUTES_PER_MATCH, MAX_PER_CLUB
    );

    for section in sections {
        let mut rows: Vec<&Candidate> = candidates
            .iter()
            .filter(|c| section.is_none_or(|pos| c.player.element_type == pos))
            .collect();
        sort_candidates(&mut rows, &args.sort);

        if let Some(pos) = section.and_then(Position::from_element_type_id) {
            println!("{}", pos.display_name().bold());
        }
        if rows.is_empty() {
            println!("No players match.\n");
            continue;
        }
        print_table(&rows, args.limit, &team_map);
        println!();
    }

    if !outs.is_empty() {
        println!(
            "Note: budget uses current prices; your actual selling price may be lower. Clubs with {} players are checked per transfer.",
            MAX_PER_CLUB
        );
    }

    Ok(())
}

fn sort_candidates(rows: &mut [&Candidate], sort: &BuySortBy) {
    rows.sort_by(|a, b| {
        let primary = match sort {
            BuySortBy::Fdr => a
                .avg_fdr
                .unwrap_or(f64::MAX)
                .total_cmp(&b.avg_fdr.unwrap_or(f64::MAX)),
            BuySortBy::Form => b.form.total_cmp(&a.form),
            BuySortBy::Pts => b.player.total_points.cmp(&a.player.total_points),
            BuySortBy::Xgi => b.xgi_per_90.total_cmp(&a.xgi_per_90),
        };
        primary.then_with(|| b.form.total_cmp(&a.form))
    });
}

fn print_table(rows: &[&Candidate], limit: usize, team_map: &HashMap<u64, String>) {
    println!(
        "{:>id_w$}  {:<pos_w$}  {:<name_w$}  {:<team_w$}  {:>cost_w$}  {:>form_w$}  {:>pts_w$}  {:>stat_w$}  {:>min_w$}  {:>avg_w$}  Fixtures",
        "ID",
        "Pos",
        "Name",
        "Team",
        "Cost",
        "Form",
        "Pts",
        "xGI/90",
        "Mins",
        "FDR",
        id_w = WIDTH_ID,
        pos_w = WIDTH_POS,
        name_w = WIDTH_NAME,
        team_w = WIDTH_TEAM_SHORT_NAME,
        cost_w = WIDTH_COST,
        form_w = WIDTH_FORM,
        pts_w = WIDTH_PTS,
        stat_w = WIDTH_STAT,
        min_w = WIDTH_MINUTES,
        avg_w = WIDTH_AVG,
    );

    for c in rows.iter().take(limit) {
        let player = c.player;
        let pos_name = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("???");
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("???");

        let fdr_padded = format!(
            "{:>width$}",
            c.avg_fdr
                .map(|avg| format!("{:.1}", avg))
                .unwrap_or_else(|| "-".to_string()),
            width = WIDTH_AVG
        );
        let fdr_display = match c.avg_fdr {
            Some(avg) if avg <= EASY_FDR_THRESHOLD => fdr_padded.green().to_string(),
            Some(avg) if avg >= TOUGH_FDR_THRESHOLD => fdr_padded.red().to_string(),
            _ => fdr_padded,
        };

        println!(
            "{:>id_w$}  {:<pos_w$}  {:<name_w$}  {:<team_w$}  {:>cost_w$.1}  {:>form_w$}  {:>pts_w$}  {:>stat_w$.2}  {:>min_w$.0}  {}  {}",
            player.id,
            pos_name,
            player.web_name,
            team_name,
            player.now_cost as f64 / 10.0,
            player.form,
            player.total_points,
            c.xgi_per_90,
            c.minutes_per_match,
            fdr_display,
            format_upcoming_fixtures(&c.fixtures),
            id_w = WIDTH_ID,
            pos_w = WIDTH_POS,
            name_w = WIDTH_NAME,
            team_w = WIDTH_TEAM_SHORT_NAME,
            cost_w = WIDTH_COST,
            form_w = WIDTH_FORM,
            pts_w = WIDTH_PTS,
            stat_w = WIDTH_STAT,
            min_w = WIDTH_MINUTES,
        );
    }
}
