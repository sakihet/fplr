use crate::error::Result;
use crate::models::Position;
use crate::models::XaSortBy;
use crate::utils::constants::WIDTH_COST;
use crate::utils::expected_stat::{ExpectedStatSpec, StatSort, print_expected_stat_table};

pub async fn handle_xa(
    sort: XaSortBy,
    team_opt: Option<String>,
    pos_opt: Option<Position>,
    limit: usize,
) -> Result<()> {
    let sort = match sort {
        XaSortBy::Assists => StatSort::Actual,
        XaSortBy::Xa => StatSort::Expected,
        XaSortBy::Diff => StatSort::Diff,
        XaSortBy::Ratio => StatSort::Ratio,
    };
    let spec = ExpectedStatSpec {
        actual_label: "A",
        expected_label: "xA",
        actual_width: WIDTH_COST,
        actual_fn: |p| p.assists as f64,
        expected_fn: |p| p.expected_assists.parse().unwrap_or(0.0),
    };

    print_expected_stat_table(sort, team_opt, pos_opt, limit, spec).await
}
