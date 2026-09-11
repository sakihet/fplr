use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, SetPieceType};
use crate::utils::constants::{
    WIDTH_NAME, WIDTH_ORDER, WIDTH_SET_PIECE_TYPE, WIDTH_TEAM_NAME, WIDTH_TEAM_SHORT_NAME,
};
use crate::utils::formatters::truncate;
use crate::utils::team_helpers::find_team_ids_by_name;

/// A set piece type, its column label, and the element field holding its taker order.
type SetPieceColumn = (SetPieceType, &'static str, fn(&Element) -> Option<u64>);

/// Set piece types, in the order they are listed for each team.
const SET_PIECE_TYPES: [SetPieceColumn; 3] = [
    (SetPieceType::Pen, "PEN", |e| e.penalties_order),
    (SetPieceType::Fk, "FK", |e| e.direct_freekicks_order),
    (SetPieceType::Corner, "CORNER", |e| {
        e.corners_and_indirect_freekicks_order
    }),
];

pub async fn handle_set_piece(team_name: Option<String>, kind: Option<SetPieceType>) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let mut teams: Vec<_> = data.teams.iter().collect();

    if let Some(ref name) = team_name {
        let ids = find_team_ids_by_name(&data.teams, name);
        if ids.is_empty() {
            println!("No team found matching '{}'", name);
            return Ok(());
        }
        teams.retain(|t| ids.contains(&t.id));
    }

    teams.sort_by(|a, b| a.name.cmp(&b.name));

    println!(
        "{:<team_w$}  {:<name_w$}  {:<type_w$}  {:>order_w$}  Player",
        "Team",
        "Name",
        "Type",
        "Order",
        team_w = WIDTH_TEAM_SHORT_NAME,
        name_w = WIDTH_TEAM_NAME,
        type_w = WIDTH_SET_PIECE_TYPE,
        order_w = WIDTH_ORDER,
    );

    let mut found = false;

    for team in teams {
        for (set_piece_type, label, order_of) in SET_PIECE_TYPES {
            if kind.as_ref().is_some_and(|k| *k != set_piece_type) {
                continue;
            }

            // Collect the takers of this set piece type, best-ranked first
            let mut takers: Vec<(u64, &Element)> = data
                .elements
                .iter()
                .filter(|e| e.team == team.id)
                .filter_map(|e| order_of(e).map(|order| (order, e)))
                .collect();
            takers.sort_by_key(|(order, _)| *order);

            for (order, element) in takers {
                found = true;
                println!(
                    "{:<team_w$}  {:<name_w$}  {:<type_w$}  {:>order_w$}  {}",
                    team.short_name,
                    truncate(&team.name, WIDTH_TEAM_NAME),
                    label,
                    order,
                    truncate(&element.web_name, WIDTH_NAME),
                    team_w = WIDTH_TEAM_SHORT_NAME,
                    name_w = WIDTH_TEAM_NAME,
                    type_w = WIDTH_SET_PIECE_TYPE,
                    order_w = WIDTH_ORDER,
                );
            }
        }
    }

    if !found {
        println!("No set piece takers match the given filters.");
    }

    Ok(())
}
