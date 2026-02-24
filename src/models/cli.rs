use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum TeamSortBy {
    #[default]
    #[value(alias = "pos")]
    Position,
    #[value(alias = "str")]
    Strength,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum TeamFormSortBy {
    #[default]
    #[value(alias = "total")]
    Total,
    #[value(alias = "fwd")]
    Forward,
    #[value(alias = "mid")]
    Midfielder,
    #[value(alias = "def")]
    Defender,
    #[value(alias = "gkp", alias = "gk")]
    Goalkeeper,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum SortBy {
    Cost,
    SelectedBy,
    Form,
    #[value(alias = "minutes-played")]
    Minutes,
    #[value(alias = "goals")]
    GoalsScored,
    #[value(alias = "assists")]
    Assists,
    #[value(alias = "clean-sheets")]
    CleanSheets,
    #[value(alias = "goals-conceded")]
    GoalsConceded,
    #[value(alias = "own-goals")]
    OwnGoals,
    #[value(alias = "penalties-saved")]
    PenaltiesSaved,
    #[value(alias = "penalties-missed")]
    PenaltiesMissed,
    #[value(alias = "yellow-cards")]
    YellowCards,
    #[value(alias = "red-cards")]
    RedCards,
    #[value(alias = "saves")]
    Saves,
    #[value(alias = "bonus")]
    Bonus,
    #[value(alias = "bps")]
    Bps,
    #[value(alias = "influence")]
    Influence,
    #[value(alias = "creativity")]
    Creativity,
    #[value(alias = "threat")]
    Threat,
    #[value(alias = "ict-index")]
    IctIndex,
    #[value(alias = "dream-team")]
    DreamTeamCount,
    #[value(alias = "value-form")]
    ValueForm,
    #[value(alias = "value-season")]
    ValueSeason,
    #[value(alias = "points-per-game")]
    PointsPerGame,
    #[value(alias = "xg")]
    ExpectedGoals,
    #[value(alias = "xa")]
    ExpectedAssists,
    #[value(alias = "xgi")]
    ExpectedGoalInvolvements,
    #[value(alias = "xgc")]
    ExpectedGoalsConceded,
    #[value(alias = "starts")]
    Starts,
    #[value(alias = "tackles")]
    Tackles,
    #[value(alias = "cbi")]
    ClearancesBlocksInterceptions,
    #[value(alias = "recoveries")]
    Recoveries,
    #[value(alias = "defensive-contribution")]
    DefensiveContribution,
    #[value(alias = "ti")]
    TransfersIn,
    #[value(alias = "to")]
    TransfersOut,
    #[value(alias = "tie")]
    TransfersInEvent,
    #[value(alias = "toe")]
    TransfersOutEvent,
    #[value(alias = "pre", alias = "price-rise-event")]
    PriceRiseEvent,
    #[value(alias = "prs", alias = "price-rise-start")]
    PriceRiseStart,
    #[default]
    Points,
}

impl SortBy {
    pub fn stat_label(&self) -> Option<&'static str> {
        match self {
            SortBy::Minutes => Some("MP"),
            SortBy::GoalsScored => Some("G"),
            SortBy::Assists => Some("A"),
            SortBy::CleanSheets => Some("CS"),
            SortBy::GoalsConceded => Some("GC"),
            SortBy::OwnGoals => Some("OG"),
            SortBy::PenaltiesSaved => Some("PS"),
            SortBy::PenaltiesMissed => Some("PM"),
            SortBy::YellowCards => Some("YC"),
            SortBy::RedCards => Some("RC"),
            SortBy::Saves => Some("S"),
            SortBy::Bonus => Some("B"),
            SortBy::Bps => Some("BPS"),
            SortBy::Influence => Some("INF"),
            SortBy::Creativity => Some("CRE"),
            SortBy::Threat => Some("THR"),
            SortBy::IctIndex => Some("ICT"),
            SortBy::DreamTeamCount => Some("DT"),
            SortBy::ValueForm => Some("V-F"),
            SortBy::ValueSeason => Some("V-S"),
            SortBy::PointsPerGame => Some("PPG"),
            SortBy::ExpectedGoals => Some("xG"),
            SortBy::ExpectedAssists => Some("xA"),
            SortBy::ExpectedGoalInvolvements => Some("xGI"),
            SortBy::ExpectedGoalsConceded => Some("xGC"),
            SortBy::Starts => Some("STR"),
            SortBy::Tackles => Some("TCK"),
            SortBy::ClearancesBlocksInterceptions => Some("CBI"),
            SortBy::Recoveries => Some("REC"),
            SortBy::DefensiveContribution => Some("DEF"),
            SortBy::TransfersIn => Some("TI"),
            SortBy::TransfersOut => Some("TO"),
            SortBy::TransfersInEvent => Some("TIE"),
            SortBy::TransfersOutEvent => Some("TOE"),
            SortBy::PriceRiseEvent => Some("CCE"),
            SortBy::PriceRiseStart => Some("CCS"),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash, Serialize, Deserialize)]
pub enum PlayerStatus {
    #[serde(rename = "a")]
    Available,
    #[serde(rename = "d")]
    Doubtful,
    #[serde(rename = "i")]
    Injured,
    #[serde(rename = "s")]
    Suspended,
    #[serde(rename = "u")]
    Unavailable,
    #[serde(rename = "n")]
    NotAvailable,
    #[serde(other)]
    Unknown,
}

impl PlayerStatus {
    pub fn is_available(&self, chance: Option<u64>) -> bool {
        match self {
            Self::Available => true,
            _ => chance == Some(100),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Doubtful => "Doubtful",
            Self::Injured => "Injured",
            Self::Suspended => "Suspended",
            Self::Unavailable => "Unavailable",
            Self::NotAvailable => "Not Available",
            Self::Unknown => "Unknown",
        }
    }
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

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum XgSortBy {
    #[value(alias = "goals")]
    Goals,
    #[default]
    #[value(alias = "xg")]
    Xg,
    #[value(alias = "diff")]
    Diff,
    #[value(alias = "ratio")]
    Ratio,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum XaSortBy {
    #[value(alias = "assists")]
    Assists,
    #[default]
    #[value(alias = "xa")]
    Xa,
    #[value(alias = "diff")]
    Diff,
    #[value(alias = "ratio")]
    Ratio,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum XgiSortBy {
    #[value(alias = "actual")]
    Actual,
    #[default]
    #[value(alias = "xgi")]
    Xgi,
    #[value(alias = "diff")]
    Diff,
    #[value(alias = "ratio")]
    Ratio,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum XgcSortBy {
    #[value(alias = "goals")]
    Goals,
    #[default]
    #[value(alias = "xgc")]
    Xgc,
    #[value(alias = "diff")]
    Diff,
    #[value(alias = "ratio")]
    Ratio,
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
