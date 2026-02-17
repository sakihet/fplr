use deunicode::deunicode;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position, SortBy};
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};

#[derive(Debug, Default)]
pub struct PlayerFilterArgs {
    pub sort: SortBy,
    pub position: Option<Position>,
    pub limit: usize,
    pub team: Option<String>,
    pub name: Option<String>,
    pub min_cost: Option<f64>,
    pub max_cost: Option<f64>,
    pub available: bool,
}

pub async fn handle_player(args: PlayerFilterArgs) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let team_map = create_team_short_name_map(&data.teams);
    let target_team_ids = if let Some(ref team_name) = args.team {
        find_team_ids_by_name(&data.teams, team_name)
    } else {
        Vec::new()
    };

    let mut players: Vec<Element> = data
        .elements
        .into_iter()
        .filter(|player| {
            let position_match = if let Some(ref pos) = args.position {
                player.element_type == pos.element_type_id() as u64
            } else {
                true
            };
            let team_match = if args.team.is_some() {
                target_team_ids.contains(&player.team)
            } else {
                true
            };
            let name_match = if let Some(ref n) = args.name {
                let normalized_player_name = deunicode(&player.web_name).to_lowercase();
                let normalized_query = deunicode(n).to_lowercase();
                normalized_player_name.contains(&normalized_query)
            } else {
                true
            };
            let cost_match = {
                let p_cost = player.now_cost as f64;
                let min_match = if let Some(min) = args.min_cost {
                    p_cost >= min * 10.0
                } else {
                    true
                };
                let max_match = if let Some(max) = args.max_cost {
                    p_cost <= max * 10.0
                } else {
                    true
                };
                min_match && max_match
            };
            let available_match = if args.available {
                player
                    .status
                    .is_available(player.chance_of_playing_next_round)
            } else {
                true
            };
            position_match && team_match && name_match && cost_match && available_match
        })
        .collect();

    match args.sort {
        SortBy::Cost => players.sort_by(|a, b| b.now_cost.cmp(&a.now_cost)),
        SortBy::Form => players.sort_by(|a, b| {
            let form_a = a.form.parse::<f64>().unwrap_or(0.0);
            let form_b = b.form.parse::<f64>().unwrap_or(0.0);
            form_b.partial_cmp(&form_a).unwrap()
        }),
        SortBy::Minutes => players.sort_by(|a, b| b.minutes.cmp(&a.minutes)),
        SortBy::GoalsScored => players.sort_by(|a, b| b.goals_scored.cmp(&a.goals_scored)),
        SortBy::Assists => players.sort_by(|a, b| b.assists.cmp(&a.assists)),
        SortBy::CleanSheets => players.sort_by(|a, b| b.clean_sheets.cmp(&a.clean_sheets)),
        SortBy::GoalsConceded => players.sort_by(|a, b| b.goals_conceded.cmp(&a.goals_conceded)),
        SortBy::OwnGoals => players.sort_by(|a, b| b.own_goals.cmp(&a.own_goals)),
        SortBy::PenaltiesSaved => players.sort_by(|a, b| b.penalties_saved.cmp(&a.penalties_saved)),
        SortBy::PenaltiesMissed => {
            players.sort_by(|a, b| b.penalties_missed.cmp(&a.penalties_missed))
        }
        SortBy::YellowCards => players.sort_by(|a, b| b.yellow_cards.cmp(&a.yellow_cards)),
        SortBy::RedCards => players.sort_by(|a, b| b.red_cards.cmp(&a.red_cards)),
        SortBy::Saves => players.sort_by(|a, b| b.saves.cmp(&a.saves)),
        SortBy::Bonus => players.sort_by(|a, b| b.bonus.cmp(&a.bonus)),
        SortBy::Bps => players.sort_by(|a, b| b.bps.cmp(&a.bps)),
        SortBy::Influence => players.sort_by(|a, b| {
            let influence_a = a.influence.parse::<f64>().unwrap_or(0.0);
            let influence_b = b.influence.parse::<f64>().unwrap_or(0.0);
            influence_b.partial_cmp(&influence_a).unwrap()
        }),
        SortBy::Creativity => players.sort_by(|a, b| {
            let creativity_a = a.creativity.parse::<f64>().unwrap_or(0.0);
            let creativity_b = b.creativity.parse::<f64>().unwrap_or(0.0);
            creativity_b.partial_cmp(&creativity_a).unwrap()
        }),
        SortBy::Threat => players.sort_by(|a, b| {
            let threat_a = a.threat.parse::<f64>().unwrap_or(0.0);
            let threat_b = b.threat.parse::<f64>().unwrap_or(0.0);
            threat_b.partial_cmp(&threat_a).unwrap()
        }),
        SortBy::IctIndex => players.sort_by(|a, b| {
            let ict_index_a = a.ict_index.parse::<f64>().unwrap_or(0.0);
            let ict_index_b = b.ict_index.parse::<f64>().unwrap_or(0.0);
            ict_index_b.partial_cmp(&ict_index_a).unwrap()
        }),
        SortBy::DreamTeamCount => players.sort_by(|a, b| b.dreamteam_count.cmp(&a.dreamteam_count)),
        SortBy::ValueForm => players.sort_by(|a, b| {
            let val_a = a.value_form.parse::<f64>().unwrap_or(0.0);
            let val_b = b.value_form.parse::<f64>().unwrap_or(0.0);
            val_b.partial_cmp(&val_a).unwrap()
        }),
        SortBy::ValueSeason => players.sort_by(|a, b| {
            let val_a = a.value_season.parse::<f64>().unwrap_or(0.0);
            let val_b = b.value_season.parse::<f64>().unwrap_or(0.0);
            val_b.partial_cmp(&val_a).unwrap()
        }),
        SortBy::PointsPerGame => players.sort_by(|a, b| {
            let ppg_a = a.points_per_game.parse::<f64>().unwrap_or(0.0);
            let ppg_b = b.points_per_game.parse::<f64>().unwrap_or(0.0);
            ppg_b.partial_cmp(&ppg_a).unwrap()
        }),
        SortBy::ExpectedGoals => players.sort_by(|a, b| {
            let val_a = a.expected_goals.parse::<f64>().unwrap_or(0.0);
            let val_b = b.expected_goals.parse::<f64>().unwrap_or(0.0);
            val_b.partial_cmp(&val_a).unwrap()
        }),
        SortBy::ExpectedAssists => players.sort_by(|a, b| {
            let val_a = a.expected_assists.parse::<f64>().unwrap_or(0.0);
            let val_b = b.expected_assists.parse::<f64>().unwrap_or(0.0);
            val_b.partial_cmp(&val_a).unwrap()
        }),
        SortBy::ExpectedGoalInvolvements => players.sort_by(|a, b| {
            let val_a = a.expected_goal_involvements.parse::<f64>().unwrap_or(0.0);
            let val_b = b.expected_goal_involvements.parse::<f64>().unwrap_or(0.0);
            val_b.partial_cmp(&val_a).unwrap()
        }),
        SortBy::ExpectedGoalsConceded => players.sort_by(|a, b| {
            let val_a = a.expected_goals_conceded.parse::<f64>().unwrap_or(0.0);
            let val_b = b.expected_goals_conceded.parse::<f64>().unwrap_or(0.0);
            val_b.partial_cmp(&val_a).unwrap()
        }),
        SortBy::Starts => players.sort_by(|a, b| b.starts.cmp(&a.starts)),
        SortBy::Tackles => players.sort_by(|a, b| b.tackles.cmp(&a.tackles)),
        SortBy::ClearancesBlocksInterceptions => players.sort_by(|a, b| {
            b.clearances_blocks_interceptions
                .cmp(&a.clearances_blocks_interceptions)
        }),
        SortBy::Recoveries => players.sort_by(|a, b| b.recoveries.cmp(&a.recoveries)),
        SortBy::DefensiveContribution => {
            players.sort_by(|a, b| b.defensive_contribution.cmp(&a.defensive_contribution))
        }
        SortBy::Points => players.sort_by(|a, b| b.total_points.cmp(&a.total_points)),
        SortBy::SelectedBy => players.sort_by(|a, b| {
            let selected_by_a = a.selected_by_percent.parse::<f64>().unwrap_or(0.0);
            let selected_by_b = b.selected_by_percent.parse::<f64>().unwrap_or(0.0);
            selected_by_b.partial_cmp(&selected_by_a).unwrap()
        }),
    }

    let is_stat_sort = matches!(
        args.sort,
        SortBy::Minutes
            | SortBy::GoalsScored
            | SortBy::Assists
            | SortBy::CleanSheets
            | SortBy::GoalsConceded
            | SortBy::OwnGoals
            | SortBy::PenaltiesSaved
            | SortBy::PenaltiesMissed
            | SortBy::YellowCards
            | SortBy::RedCards
            | SortBy::Saves
            | SortBy::Bonus
            | SortBy::Bps
            | SortBy::Influence
            | SortBy::Creativity
            | SortBy::Threat
            | SortBy::IctIndex
            | SortBy::DreamTeamCount
            | SortBy::ValueForm
            | SortBy::ValueSeason
            | SortBy::PointsPerGame
            | SortBy::ExpectedGoals
            | SortBy::ExpectedAssists
            | SortBy::ExpectedGoalInvolvements
            | SortBy::ExpectedGoalsConceded
            | SortBy::Starts
            | SortBy::Tackles
            | SortBy::ClearancesBlocksInterceptions
            | SortBy::Recoveries
            | SortBy::DefensiveContribution
    );

    if is_stat_sort {
        let stat_label = match args.sort {
            SortBy::Minutes => "MP",
            SortBy::GoalsScored => "G",
            SortBy::Assists => "A",
            SortBy::CleanSheets => "CS",
            SortBy::GoalsConceded => "GC",
            SortBy::OwnGoals => "OG",
            SortBy::PenaltiesSaved => "PS",
            SortBy::PenaltiesMissed => "PM",
            SortBy::YellowCards => "YC",
            SortBy::RedCards => "RC",
            SortBy::Saves => "S",
            SortBy::Bonus => "B",
            SortBy::Bps => "BPS",
            SortBy::Influence => "INF",
            SortBy::Creativity => "CRE",
            SortBy::Threat => "THR",
            SortBy::IctIndex => "ICT",
            SortBy::DreamTeamCount => "DT",
            SortBy::ValueForm => "V-F",
            SortBy::ValueSeason => "V-S",
            SortBy::PointsPerGame => "PPG",
            SortBy::ExpectedGoals => "xG",
            SortBy::ExpectedAssists => "xA",
            SortBy::ExpectedGoalInvolvements => "xGI",
            SortBy::ExpectedGoalsConceded => "xGC",
            SortBy::Starts => "STR",
            SortBy::Tackles => "TCK",
            SortBy::ClearancesBlocksInterceptions => "CBI",
            SortBy::Recoveries => "REC",
            SortBy::DefensiveContribution => "DEF",
            _ => "",
        };
        println!(
            "{:<4} {:<20} {:<4} {:<6} {:<6} {:<8} {:<6} {:<8} {:<6}",
            "ID", "Name", "Pos", "Team", "Cost", "Selected", "Form", "Points", stat_label
        );
    } else {
        println!(
            "{:<4} {:<20} {:<4} {:<6} {:<6} {:<8} {:<6} {:<8} {:<30}",
            "ID", "Name", "Pos", "Team", "Cost", "Selected", "Form", "Points", "News"
        );
    }

    for player in players.iter().take(args.limit) {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        if is_stat_sort {
            let stat_value = match args.sort {
                SortBy::Minutes => player.minutes.to_string(),
                SortBy::GoalsScored => player.goals_scored.to_string(),
                SortBy::Assists => player.assists.to_string(),
                SortBy::CleanSheets => player.clean_sheets.to_string(),
                SortBy::GoalsConceded => player.goals_conceded.to_string(),
                SortBy::OwnGoals => player.own_goals.to_string(),
                SortBy::PenaltiesSaved => player.penalties_saved.to_string(),
                SortBy::PenaltiesMissed => player.penalties_missed.to_string(),
                SortBy::YellowCards => player.yellow_cards.to_string(),
                SortBy::RedCards => player.red_cards.to_string(),
                SortBy::Saves => player.saves.to_string(),
                SortBy::Bonus => player.bonus.to_string(),
                SortBy::Bps => player.bps.to_string(),
                SortBy::Influence => player.influence.clone(),
                SortBy::Creativity => player.creativity.clone(),
                SortBy::Threat => player.threat.clone(),
                SortBy::IctIndex => player.ict_index.clone(),
                SortBy::DreamTeamCount => player.dreamteam_count.to_string(),
                SortBy::ValueForm => player.value_form.clone(),
                SortBy::ValueSeason => player.value_season.clone(),
                SortBy::PointsPerGame => player.points_per_game.clone(),
                SortBy::ExpectedGoals => player.expected_goals.clone(),
                SortBy::ExpectedAssists => player.expected_assists.clone(),
                SortBy::ExpectedGoalInvolvements => player.expected_goal_involvements.clone(),
                SortBy::ExpectedGoalsConceded => player.expected_goals_conceded.clone(),
                SortBy::Starts => player.starts.to_string(),
                SortBy::Tackles => player.tackles.to_string(),
                SortBy::ClearancesBlocksInterceptions => {
                    player.clearances_blocks_interceptions.to_string()
                }
                SortBy::Recoveries => player.recoveries.to_string(),
                SortBy::DefensiveContribution => player.defensive_contribution.to_string(),
                _ => "".to_string(),
            };
            println!(
                "{:<4} {:<20} {:<4} {:<6} {:<6} {:<8} {:<6} {:<8} {:<6}",
                player.id,
                player.web_name,
                Position::from_element_type_id(player.element_type)
                    .map(|p| p.display_name().to_string())
                    .unwrap_or("N/A".to_string()),
                team_name,
                format!("{:.1}", player.now_cost as f64 / 10.0),
                player.selected_by_percent,
                player.form,
                player.total_points,
                stat_value,
            );
        } else {
            println!(
                "{:<4} {:<20} {:<4} {:<6} {:<6} {:<8} {:<6} {:<8} {:<30}",
                player.id,
                player.web_name,
                Position::from_element_type_id(player.element_type)
                    .map(|p| p.display_name().to_string())
                    .unwrap_or("N/A".to_string()),
                team_name,
                format!("{:.1}", player.now_cost as f64 / 10.0),
                player.selected_by_percent,
                player.form,
                player.total_points,
                player.news,
            );
        }
    }

    Ok(())
}
