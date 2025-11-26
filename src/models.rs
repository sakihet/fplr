use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, ValueEnum)]
pub enum SortBy {
    Cost,
    SelectedBy,
    Form,
    Points,
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::Points
    }
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
    pub total_points: u64,
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
