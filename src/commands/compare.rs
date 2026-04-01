use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::models::Position;
use crate::utils::team_helpers::create_team_map;
use owo_colors::OwoColorize;

pub async fn handle_compare(id1: u64, id2: u64) -> Result<()> {
    let bootstrap = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_map(&bootstrap.teams);

    let p1 = bootstrap
        .elements
        .iter()
        .find(|e| e.id == id1)
        .ok_or(FplrError::PlayerNotFound(id1))?;
    let p2 = bootstrap
        .elements
        .iter()
        .find(|e| e.id == id2)
        .ok_or(FplrError::PlayerNotFound(id2))?;

    let name_w = 20;
    let label_w = 25;

    println!(
        "\n{:<label_w$} {:>name_w$} {:>name_w$}",
        "",
        p1.web_name,
        p2.web_name,
        label_w = label_w,
        name_w = name_w
    );
    println!("{}", "=".repeat(label_w + name_w * 2 + 2));

    // Helper for rows
    let print_row = |label: &str, val1: &str, val2: &str, highlight: Option<i8>| {
        // highlight: -1 (val1 is better), 1 (val2 is better), 0 (none)
        let v1_str = match highlight {
            Some(-1) => val1.green().bold().to_string(),
            _ => val1.to_string(),
        };
        let v2_str = match highlight {
            Some(1) => val2.green().bold().to_string(),
            _ => val2.to_string(),
        };

        let l_pad = label_w.saturating_sub(label.chars().count());
        let v1_pad = name_w.saturating_sub(val1.chars().count());
        let v2_pad = name_w.saturating_sub(val2.chars().count());

        println!(
            "{}{} {}{} {}{}",
            label.dimmed(),
            " ".repeat(l_pad),
            " ".repeat(v1_pad),
            v1_str,
            " ".repeat(v2_pad),
            v2_str,
        );
    };

    let p1_team = team_map.get(&p1.team).map(|s| s.as_str()).unwrap_or("???");
    let p2_team = team_map.get(&p2.team).map(|s| s.as_str()).unwrap_or("???");
    print_row("Team", p1_team, p2_team, None);

    let p1_pos = Position::from_element_type_id(p1.element_type)
        .map(|p| p.display_name())
        .unwrap_or("???");
    let p2_pos = Position::from_element_type_id(p2.element_type)
        .map(|p| p.display_name())
        .unwrap_or("???");
    print_row("Position", p1_pos, p2_pos, None);

    // Cost (Lower is better)
    let c1 = p1.now_cost as f64 / 10.0;
    let c2 = p2.now_cost as f64 / 10.0;
    print_row(
        "Cost",
        &format!("£{:.1}m", c1),
        &format!("£{:.1}m", c2),
        match c1 {
            c if c < c2 => Some(-1),
            c if c > c2 => Some(1),
            _ => None,
        },
    );

    // Selected By %
    let s1 = p1.selected_by_percent.parse::<f64>().unwrap_or(0.0);
    let s2 = p2.selected_by_percent.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Selected By %",
        &format!("{}%", p1.selected_by_percent),
        &format!("{}%", p2.selected_by_percent),
        match s1 {
            s if s > s2 => Some(-1),
            s if s < s2 => Some(1),
            _ => None,
        },
    );

    // Form
    let f1 = p1.form.parse::<f64>().unwrap_or(0.0);
    let f2 = p2.form.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Form",
        &p1.form,
        &p2.form,
        match f1 {
            f if f > f2 => Some(-1),
            f if f < f2 => Some(1),
            _ => None,
        },
    );

    // PPG
    let ppg1 = p1.points_per_game.parse::<f64>().unwrap_or(0.0);
    let ppg2 = p2.points_per_game.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Points / Game",
        &p1.points_per_game,
        &p2.points_per_game,
        match ppg1 {
            p if p > ppg2 => Some(-1),
            p if p < ppg2 => Some(1),
            _ => None,
        },
    );

    // Total Points
    print_row(
        "Total Points",
        &p1.total_points.to_string(),
        &p2.total_points.to_string(),
        match p1.total_points {
            p if p > p2.total_points => Some(-1),
            p if p < p2.total_points => Some(1),
            _ => None,
        },
    );

    println!("\nExpected Stats:");
    // xG
    let xg1 = p1.expected_goals.parse::<f64>().unwrap_or(0.0);
    let xg2 = p2.expected_goals.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Expected Goals (xG)",
        &p1.expected_goals,
        &p2.expected_goals,
        match xg1 {
            x if x > xg2 => Some(-1),
            x if x < xg2 => Some(1),
            _ => None,
        },
    );

    // xA
    let xa1 = p1.expected_assists.parse::<f64>().unwrap_or(0.0);
    let xa2 = p2.expected_assists.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Expected Assists (xA)",
        &p1.expected_assists,
        &p2.expected_assists,
        match xa1 {
            x if x > xa2 => Some(-1),
            x if x < xa2 => Some(1),
            _ => None,
        },
    );

    // xGI
    let xgi1 = p1.expected_goal_involvements.parse::<f64>().unwrap_or(0.0);
    let xgi2 = p2.expected_goal_involvements.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Expected GI (xGI)",
        &p1.expected_goal_involvements,
        &p2.expected_goal_involvements,
        match xgi1 {
            x if x > xgi2 => Some(-1),
            x if x < xgi2 => Some(1),
            _ => None,
        },
    );

    // xGC (Lower is better)
    let xgc1 = p1.expected_goals_conceded.parse::<f64>().unwrap_or(0.0);
    let xgc2 = p2.expected_goals_conceded.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Expected GC (xGC)",
        &p1.expected_goals_conceded,
        &p2.expected_goals_conceded,
        match xgc1 {
            x if x < xgc2 => Some(-1),
            x if x > xgc2 => Some(1),
            _ => None,
        },
    );

    println!("\nAttacking Stats:");
    print_row(
        "Goals",
        &p1.goals_scored.to_string(),
        &p2.goals_scored.to_string(),
        match p1.goals_scored {
            g if g > p2.goals_scored => Some(-1),
            g if g < p2.goals_scored => Some(1),
            _ => None,
        },
    );
    print_row(
        "Assists",
        &p1.assists.to_string(),
        &p2.assists.to_string(),
        match p1.assists {
            a if a > p2.assists => Some(-1),
            a if a < p2.assists => Some(1),
            _ => None,
        },
    );

    // Threat
    let t1 = p1.threat.parse::<f64>().unwrap_or(0.0);
    let t2 = p2.threat.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Threat",
        &p1.threat,
        &p2.threat,
        match t1 {
            t if t > t2 => Some(-1),
            t if t < t2 => Some(1),
            _ => None,
        },
    );

    // Creativity
    let cr1 = p1.creativity.parse::<f64>().unwrap_or(0.0);
    let cr2 = p2.creativity.parse::<f64>().unwrap_or(0.0);
    print_row(
        "Creativity",
        &p1.creativity,
        &p2.creativity,
        match cr1 {
            c if c > cr2 => Some(-1),
            c if c < cr2 => Some(1),
            _ => None,
        },
    );

    // ICT
    let ict1 = p1.ict_index.parse::<f64>().unwrap_or(0.0);
    let ict2 = p2.ict_index.parse::<f64>().unwrap_or(0.0);
    print_row(
        "ICT Index",
        &p1.ict_index,
        &p2.ict_index,
        match ict1 {
            i if i > ict2 => Some(-1),
            i if i < ict2 => Some(1),
            _ => None,
        },
    );

    println!("\nDefensive Stats:");
    print_row(
        "Clean Sheets",
        &p1.clean_sheets.to_string(),
        &p2.clean_sheets.to_string(),
        match p1.clean_sheets {
            c if c > p2.clean_sheets => Some(-1),
            c if c < p2.clean_sheets => Some(1),
            _ => None,
        },
    );
    // Goals Conceded (Lower is better)
    print_row(
        "Goals Conceded",
        &p1.goals_conceded.to_string(),
        &p2.goals_conceded.to_string(),
        match p1.goals_conceded {
            g if g < p2.goals_conceded => Some(-1),
            g if g > p2.goals_conceded => Some(1),
            _ => None,
        },
    );
    print_row(
        "Saves",
        &p1.saves.to_string(),
        &p2.saves.to_string(),
        match p1.saves {
            s if s > p2.saves => Some(-1),
            s if s < p2.saves => Some(1),
            _ => None,
        },
    );
    print_row(
        "Tackles",
        &p1.tackles.to_string(),
        &p2.tackles.to_string(),
        match p1.tackles {
            t if t > p2.tackles => Some(-1),
            t if t < p2.tackles => Some(1),
            _ => None,
        },
    );
    print_row(
        "Recoveries",
        &p1.recoveries.to_string(),
        &p2.recoveries.to_string(),
        match p1.recoveries {
            r if r > p2.recoveries => Some(-1),
            r if r < p2.recoveries => Some(1),
            _ => None,
        },
    );
    print_row(
        "Clr / Blk / Int",
        &p1.clearances_blocks_interceptions.to_string(),
        &p2.clearances_blocks_interceptions.to_string(),
        match p1.clearances_blocks_interceptions {
            c if c > p2.clearances_blocks_interceptions => Some(-1),
            c if c < p2.clearances_blocks_interceptions => Some(1),
            _ => None,
        },
    );

    println!("\nSeason Totals:");
    print_row(
        "Minutes",
        &p1.minutes.to_string(),
        &p2.minutes.to_string(),
        match p1.minutes {
            m if m > p2.minutes => Some(-1),
            m if m < p2.minutes => Some(1),
            _ => None,
        },
    );
    print_row(
        "Bonus",
        &p1.bonus.to_string(),
        &p2.bonus.to_string(),
        match p1.bonus {
            b if b > p2.bonus => Some(-1),
            b if b < p2.bonus => Some(1),
            _ => None,
        },
    );
    print_row(
        "BPS",
        &p1.bps.to_string(),
        &p2.bps.to_string(),
        match p1.bps {
            b if b > p2.bps => Some(-1),
            b if b < p2.bps => Some(1),
            _ => None,
        },
    );
    // Yellow Cards (Lower is better)
    print_row(
        "Yellow Cards",
        &p1.yellow_cards.to_string(),
        &p2.yellow_cards.to_string(),
        match p1.yellow_cards {
            y if y < p2.yellow_cards => Some(-1),
            y if y > p2.yellow_cards => Some(1),
            _ => None,
        },
    );
    // Red Cards (Lower is better)
    print_row(
        "Red Cards",
        &p1.red_cards.to_string(),
        &p2.red_cards.to_string(),
        match p1.red_cards {
            r if r < p2.red_cards => Some(-1),
            r if r > p2.red_cards => Some(1),
            _ => None,
        },
    );

    Ok(())
}
