use crate::error::Result;
use crate::models::Position;
use crate::models::XgSortBy;
use crate::utils::constants::WIDTH_COST;
use crate::utils::expected_stat::{ExpectedStatSpec, StatSort, print_expected_stat_table};

pub async fn handle_xg(
    sort: XgSortBy,
    team_opt: Option<String>,
    pos_opt: Option<Position>,
    limit: usize,
) -> Result<()> {
    let sort = match sort {
        XgSortBy::Goals => StatSort::Actual,
        XgSortBy::Xg => StatSort::Expected,
        XgSortBy::Diff => StatSort::Diff,
        XgSortBy::Ratio => StatSort::Ratio,
    };
    let spec = ExpectedStatSpec {
        actual_label: "G",
        expected_label: "xG",
        actual_width: WIDTH_COST,
        actual_fn: |p| p.goals_scored as f64,
        expected_fn: |p| p.expected_goals.parse().unwrap_or(0.0),
    };

    print_expected_stat_table(sort, team_opt, pos_opt, limit, spec).await
}
