use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum SortBy {
    Cost,
    SelectedBy,
    Form,
    #[default]
    Points,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Position {
    pub fn element_type_id(&self) -> u8 {
        match self {
            Position::Goalkeeper => 1,
            Position::Defender => 2,
            Position::Midfielder => 3,
            Position::Forward => 4,
        }
    }
    pub fn display_name(&self) -> &str {
        match self {
            Position::Goalkeeper => "GKP",
            Position::Defender => "DEF",
            Position::Midfielder => "MID",
            Position::Forward => "FWD",
        }
    }
    pub fn from_element_type_id(id: u64) -> Option<Self> {
        match id {
            1 => Some(Position::Goalkeeper),
            2 => Some(Position::Defender),
            3 => Some(Position::Midfielder),
            4 => Some(Position::Forward),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DreamTeamTopPlayer {
    pub id: u64,
    pub points: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DreamTeamTeam {
    pub element: u64,
    pub points: i64,
    pub position: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DreamTeam {
    pub top_player: DreamTeamTopPlayer,
    pub team: Vec<DreamTeamTeam>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: u64,
    pub name: String,
    pub is_current: bool,
    pub is_next: bool,
    pub deadline_time: String,
    pub finished: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Element {
    pub id: u64,
    pub web_name: String,
    pub element_type: u64,
    pub team: u64,
    pub now_cost: u64,
    pub selected_by_percent: String,
    pub form: String,
    pub total_points: i64,
    pub news: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    pub id: u64,
    pub name: String,
    pub short_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrapStatic {
    pub events: Vec<Event>,
    pub elements: Vec<Element>,
    pub teams: Vec<Team>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerHistory {
    pub element: u64,
    pub fixture: u64,
    pub opponent_team: u64,
    pub total_points: i64,
    pub was_home: bool,
    pub kickoff_time: String,
    pub team_h_score: u64,
    pub team_a_score: u64,
    pub round: u64,
    pub modified: bool,
    pub minutes: u64,
    pub goals_scored: u64,
    pub assists: u64,
    pub clean_sheets: u64,
    pub goals_conceded: u64,
    pub own_goals: u64,
    pub penalties_saved: u64,
    pub penalties_missed: u64,
    pub yellow_cards: u64,
    pub red_cards: u64,
    pub saves: u64,
    pub bonus: u64,
    pub bps: i64,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub clearances_blocks_interceptions: u64,
    pub recoveries: u64,
    pub tackles: u64,
    pub defensive_contribution: u64,
    pub starts: u64,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
    pub value: u64,
    pub transfers_balance: i64,
    pub selected: u64,
    pub transfers_in: u64,
    pub transfers_out: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerSummary {
    pub history: Vec<PlayerHistory>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveData {
    pub elements: Vec<LiveElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveElement {
    pub id: u64,
    pub stats: LiveStats,
    pub explain: Vec<LiveExplain>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveStats {
    pub assists: u64,
    pub bonus: u64,
    pub bps: i64,
    pub clean_sheets: u64,
    pub clearances_blocks_interceptions: u64,
    pub creativity: String,
    pub defensive_contribution: u64,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals: String,
    pub expected_goals_conceded: String,
    pub goals_conceded: u64,
    pub goals_scored: u64,
    pub ict_index: String,
    pub in_dreamteam: bool,
    pub influence: String,
    pub minutes: u64,
    pub own_goals: u64,
    pub penalties_missed: u64,
    pub penalties_saved: u64,
    pub recoveries: u64,
    pub red_cards: u64,
    pub saves: u64,
    pub starts: u64,
    pub tackles: u64,
    pub threat: String,
    pub total_points: i64,
    pub yellow_cards: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveExplain {
    pub fixture: u64,
    pub stats: Vec<LiveExplainStat>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveExplainStat {
    pub identifier: String,
    pub points: i64,
    pub value: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pick {
    pub element: u64,
    pub position: u32,
    pub multiplier: u8,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub element_type: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManagerPicks {
    pub picks: Vec<Pick>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_element_type_id() {
        assert_eq!(Position::Goalkeeper.element_type_id(), 1);
        assert_eq!(Position::Defender.element_type_id(), 2);
        assert_eq!(Position::Midfielder.element_type_id(), 3);
        assert_eq!(Position::Forward.element_type_id(), 4);
    }
}
