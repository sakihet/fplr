use clap::ValueEnum;

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
