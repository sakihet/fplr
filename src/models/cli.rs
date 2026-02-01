use clap::ValueEnum;

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
    #[value(alias = "gkp", alias = "gk")]
    Goalkeeper,
    #[value(alias = "def")]
    Defender,
    #[value(alias = "mid")]
    Midfielder,
    #[value(alias = "fwd")]
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
    pub fn display_name(&self) -> &'static str {
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
