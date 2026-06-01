use crate::error::Result;
use crate::models::{Position, SortBy};

use super::player::{PlayerFilterArgs, handle_player};

pub async fn handle_differential(
    max_sel: f64,
    sort: SortBy,
    position: Option<Position>,
    limit: usize,
) -> Result<()> {
    handle_player(PlayerFilterArgs {
        sort,
        position,
        limit,
        max_sel: Some(max_sel),
        ..Default::default()
    })
    .await
}
