use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::utils::constants::{WIDTH_NAME, WIDTH_STAT};
use crate::utils::event_helpers::{find_next_event, get_effective_event_id};
use crate::utils::team_helpers::create_team_ref_map;
use clap::Args;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct SwingArgs {
    /// Starting Gameweek (defaults to current/next)
    #[arg(short, long)]
    pub gw: Option<u32>,

    /// Number of Gameweeks in each comparison window
    #[arg(short, long, default_value = "4")]
    pub window: u32,

    /// Minimum average FDR difference to be considered a swing
    #[arg(short, long, default_value = "1.0")]
    pub threshold: f32,
}

#[derive(Debug)]
struct SwingResult {
    team_name: String,
    before_avg: f32,
    after_avg: f32,
    diff: f32,
    before_gw_start: u32,
    before_gw_end: u32,
    after_gw_start: u32,
    after_gw_end: u32,
}

pub async fn handle_swing(args: SwingArgs) -> Result<()> {
    let bootstrap_data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_ref_map(&bootstrap_data.teams);
    let fixtures = FplClient::fetch_fixtures().await?;

    // Fixtures are scheduled in advance, so before the season starts fall back to the
    // upcoming GW1 instead of erroring out.
    let start_gw = args
        .gw
        .or_else(|| crate::utils::event_helpers::get_current_event_id(&bootstrap_data.events))
        .or_else(|| get_effective_event_id(&bootstrap_data.events, None))
        .or_else(|| find_next_event(&bootstrap_data.events).map(|e| e.id as u32))
        .ok_or(FplrError::NoNextEvent)?;

    let window = args.window;
    let threshold = args.threshold;

    let before_gw_start = start_gw;
    let before_gw_end = start_gw + window - 1;
    let after_gw_start = start_gw + window;
    let after_gw_end = start_gw + window * 2 - 1;

    let mut positive_swings = Vec::new();
    let mut negative_swings = Vec::new();

    for (team_id, team) in team_map.iter() {
        let team_fixtures: Vec<_> = fixtures
            .iter()
            .filter(|f| f.event.is_some() && (f.team_h == *team_id || f.team_a == *team_id))
            .collect();

        let mut before_fdr_sum = 0;
        let mut before_count = 0;
        let mut after_fdr_sum = 0;
        let mut after_count = 0;

        for f in team_fixtures {
            let event = f.event.unwrap() as u32;
            let is_home = f.team_h == *team_id;
            let fdr = if is_home {
                f.team_h_difficulty
            } else {
                f.team_a_difficulty
            };

            if event >= before_gw_start && event <= before_gw_end {
                before_fdr_sum += fdr as u32;
                before_count += 1;
            } else if event >= after_gw_start && event <= after_gw_end {
                after_fdr_sum += fdr as u32;
                after_count += 1;
            }
        }

        // If a team has 0 fixtures in a window (blank GWs), we can't calculate a standard average.
        // For simplicity, skip teams if they have no fixtures in either window.
        if before_count == 0 || after_count == 0 {
            continue;
        }

        let before_avg = before_fdr_sum as f32 / before_count as f32;
        let after_avg = after_fdr_sum as f32 / after_count as f32;
        let diff = after_avg - before_avg;

        let result = SwingResult {
            team_name: team.short_name.clone(),
            before_avg,
            after_avg,
            diff,
            before_gw_start,
            before_gw_end,
            after_gw_start,
            after_gw_end,
        };

        if diff <= -threshold {
            positive_swings.push(result);
        } else if diff >= threshold {
            negative_swings.push(result);
        }
    }

    positive_swings.sort_by(|a, b| a.diff.partial_cmp(&b.diff).unwrap());
    negative_swings.sort_by(|a, b| b.diff.partial_cmp(&a.diff).unwrap());

    println!(
        "Fixture Swings starting at GW{} (Window: {} GWs, Threshold: {:.1})\n",
        start_gw, window, threshold
    );

    if !positive_swings.is_empty() {
        println!("{}", "🟢 [Positive Swings] (Getting Easier)".green().bold());
        print_swing_table(&positive_swings);
        println!();
    } else {
        println!(
            "{}",
            "🟢 [Positive Swings] None found matching criteria.".dimmed()
        );
        println!();
    }

    if !negative_swings.is_empty() {
        println!("{}", "🔴 [Negative Swings] (Getting Harder)".red().bold());
        print_swing_table(&negative_swings);
    } else {
        println!(
            "{}",
            "🔴 [Negative Swings] None found matching criteria.".dimmed()
        );
    }

    Ok(())
}

fn print_swing_table(results: &[SwingResult]) {
    println!(
        "{:<team_w$}  {:<name_w$}  {:<name_w$}  {:<team_w$}",
        "Team",
        "Before",
        "After",
        "Diff",
        team_w = WIDTH_STAT,
        name_w = WIDTH_NAME,
    );
    println!("{}", "-".repeat(48));
    for r in results {
        let before_str = format!(
            "{:.2} (GW{}-{})",
            r.before_avg, r.before_gw_start, r.before_gw_end
        );
        let after_str = format!(
            "{:.2} (GW{}-{})",
            r.after_avg, r.after_gw_start, r.after_gw_end
        );

        let diff_str = if r.diff > 0.0 {
            format!("+{:.2}", r.diff).red().to_string()
        } else {
            format!("{:.2}", r.diff).green().to_string()
        };

        println!(
            "{:<team_w$}  {:<name_w$}  {:<name_w$}  {}",
            r.team_name,
            before_str,
            after_str,
            diff_str,
            team_w = WIDTH_STAT,
            name_w = WIDTH_NAME,
        );
    }
}
