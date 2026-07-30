use crate::error::Result;
use crate::models::Position;
use crate::models::XgcSortBy;
use crate::utils::constants::WIDTH_COST;
use crate::utils::expected_stat::{ExpectedStatSpec, StatSort, print_expected_stat_table};

pub async fn handle_xgc(
    sort: XgcSortBy,
    team_opt: Option<String>,
    pos_opt: Option<Position>,
    limit: usize,
) -> Result<()> {
    let sort = match sort {
        XgcSortBy::Goals => StatSort::Actual,
        XgcSortBy::Xgc => StatSort::Expected,
        XgcSortBy::Diff => StatSort::Diff,
        XgcSortBy::Ratio => StatSort::Ratio,
    };
    let spec = ExpectedStatSpec {
        actual_label: "GC",
        expected_label: "xGC",
        actual_width: WIDTH_COST,
        actual_fn: |p| p.goals_conceded as f64,
        expected_fn: |p| p.expected_goals_conceded.parse().unwrap_or(0.0),
    };

    print_expected_stat_table(sort, team_opt, pos_opt, limit, spec).await
}
