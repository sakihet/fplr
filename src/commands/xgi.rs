use crate::error::Result;
use crate::models::Position;
use crate::models::XgiSortBy;
use crate::utils::constants::WIDTH_STAT;
use crate::utils::expected_stat::{ExpectedStatSpec, StatSort, print_expected_stat_table};

pub async fn handle_xgi(
    sort: XgiSortBy,
    team_opt: Option<String>,
    pos_opt: Option<Position>,
    limit: usize,
) -> Result<()> {
    let sort = match sort {
        XgiSortBy::Actual => StatSort::Actual,
        XgiSortBy::Xgi => StatSort::Expected,
        XgiSortBy::Diff => StatSort::Diff,
        XgiSortBy::Ratio => StatSort::Ratio,
    };
    let spec = ExpectedStatSpec {
        actual_label: "Actual",
        expected_label: "xGI",
        actual_width: WIDTH_STAT,
        actual_fn: |p| (p.goals_scored + p.assists) as f64,
        expected_fn: |p| p.expected_goal_involvements.parse().unwrap_or(0.0),
    };

    print_expected_stat_table(sort, team_opt, pos_opt, limit, spec).await
}
