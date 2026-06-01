use crate::api::FplClient;
use crate::error::Result;
use crate::models::Position;
use crate::utils::constants::*;
use crate::utils::formatters::truncate;
use crate::utils::team_helpers::create_team_ref_map;

const SQUAD: &[(Position, usize)] = &[
    (Position::Goalkeeper, 2),
    (Position::Defender, 5),
    (Position::Midfielder, 5),
    (Position::Forward, 3),
];

pub async fn handle_template() -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;
    let team_map = create_team_ref_map(&data.teams);

    println!(
        "{:>id_w$}  {:<name_w$}  {:<team_w$}  {:<pos_w$}  {:>cost_w$}  {:>sel_w$}  {:>pts_w$}",
        "ID",
        "Name",
        "Team",
        "Pos",
        "Cost",
        "Sel%",
        "Pts",
        id_w = WIDTH_ID,
        name_w = WIDTH_NAME,
        team_w = WIDTH_TEAM_SHORT_NAME,
        pos_w = WIDTH_POS,
        cost_w = WIDTH_COST,
        sel_w = WIDTH_SEL,
        pts_w = WIDTH_PTS,
    );

    let mut total_cost: f64 = 0.0;

    for (position, count) in SQUAD {
        let type_id = position.element_type_id() as u64;
        let pos_label = position.display_name();

        let mut players: Vec<_> = data
            .elements
            .iter()
            .filter(|e| e.element_type == type_id)
            .collect();

        players.sort_by(|a, b| {
            let a_sel: f64 = a.selected_by_percent.parse().unwrap_or(0.0);
            let b_sel: f64 = b.selected_by_percent.parse().unwrap_or(0.0);
            b_sel.partial_cmp(&a_sel).unwrap()
        });

        for player in players.iter().take(*count) {
            let team_short = team_map
                .get(&player.team)
                .map(|t| t.short_name.as_str())
                .unwrap_or("???");
            let cost = player.now_cost as f64 / 10.0;
            total_cost += cost;
            let sel: f64 = player.selected_by_percent.parse().unwrap_or(0.0);

            println!(
                "{:>id_w$}  {:<name_w$}  {:<team_w$}  {:<pos_w$}  {:>cost_w$.1}  {:>sel_w$.1}  {:>pts_w$}",
                player.id,
                truncate(&player.web_name, WIDTH_NAME),
                team_short,
                pos_label,
                cost,
                sel,
                player.total_points,
                id_w = WIDTH_ID,
                name_w = WIDTH_NAME,
                team_w = WIDTH_TEAM_SHORT_NAME,
                pos_w = WIDTH_POS,
                cost_w = WIDTH_COST,
                sel_w = WIDTH_SEL,
                pts_w = WIDTH_PTS,
            );
        }
    }

    println!("Total: {:.1}  Budget remaining: {:.1}", total_cost, 100.0 - total_cost);

    Ok(())
}
