use deunicode::deunicode;

use crate::api::FplClient;
use crate::error::Result;
use crate::models::{Element, Position, SortBy};
use crate::utils::region_helpers::find_region_id;
use crate::utils::team_helpers::{create_team_short_name_map, find_team_ids_by_name};
use crate::utils::{constants::*, formatters::*};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PlayerFilterArgs {
    pub sort: SortBy,
    pub position: Option<Position>,
    pub limit: usize,
    pub team: Option<String>,
    pub name: Option<String>,
    pub region: Option<String>,
    pub min_cost: Option<f64>,
    pub max_cost: Option<f64>,
    pub available: bool,
    pub max_sel: Option<f64>,
}

pub async fn handle_player(args: PlayerFilterArgs) -> Result<()> {
    let data = FplClient::fetch_bootstrap_static().await?;

    let team_map = create_team_short_name_map(&data.teams);
    let target_team_ids = if let Some(ref team_name) = args.team {
        find_team_ids_by_name(&data.teams, team_name)
    } else {
        Vec::new()
    };

    let target_region_id = if let Some(ref region_query) = args.region {
        find_region_id(region_query)
    } else {
        None
    };

    let mut players = filter_players(data.elements, &args, &target_team_ids, target_region_id);
    sort_players(&mut players, &args.sort);

    print_players(&players, &args, &team_map);

    Ok(())
}

fn filter_players(
    players: Vec<Element>,
    args: &PlayerFilterArgs,
    target_team_ids: &[u64],
    target_region_id: Option<u64>,
) -> Vec<Element> {
    players
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
            let region_match = if args.region.is_some() {
                if let Some(r) = target_region_id {
                    player.region == Some(r)
                } else {
                    false
                }
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
            let sel_match = if let Some(max_sel) = args.max_sel {
                player
                    .selected_by_percent
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    <= max_sel
            } else {
                true
            };
            position_match
                && team_match
                && name_match
                && region_match
                && cost_match
                && available_match
                && sel_match
        })
        .collect()
}

fn sort_players(players: &mut [Element], sort_by: &SortBy) {
    match sort_by {
        SortBy::Assists => players.sort_by(|a, b| b.assists.cmp(&a.assists)),
        SortBy::Bonus => players.sort_by(|a, b| b.bonus.cmp(&a.bonus)),
        SortBy::Bps => players.sort_by(|a, b| b.bps.cmp(&a.bps)),
        SortBy::CleanSheets => players.sort_by(|a, b| b.clean_sheets.cmp(&a.clean_sheets)),
        SortBy::ClearancesBlocksInterceptions => players.sort_by(|a, b| {
            b.clearances_blocks_interceptions
                .cmp(&a.clearances_blocks_interceptions)
        }),
        SortBy::Cost => players.sort_by(|a, b| b.now_cost.cmp(&a.now_cost)),
        SortBy::Creativity => players.sort_by(|a, b| {
            parse_f64(&b.creativity)
                .partial_cmp(&parse_f64(&a.creativity))
                .unwrap()
        }),
        SortBy::DefensiveContribution => {
            players.sort_by(|a, b| b.defensive_contribution.cmp(&a.defensive_contribution))
        }
        SortBy::DreamTeamCount => players.sort_by(|a, b| b.dreamteam_count.cmp(&a.dreamteam_count)),
        SortBy::ExpectedAssists => players.sort_by(|a, b| {
            parse_f64(&b.expected_assists)
                .partial_cmp(&parse_f64(&a.expected_assists))
                .unwrap()
        }),
        SortBy::ExpectedGoalInvolvements => players.sort_by(|a, b| {
            parse_f64(&b.expected_goal_involvements)
                .partial_cmp(&parse_f64(&a.expected_goal_involvements))
                .unwrap()
        }),
        SortBy::ExpectedGoals => players.sort_by(|a, b| {
            parse_f64(&b.expected_goals)
                .partial_cmp(&parse_f64(&a.expected_goals))
                .unwrap()
        }),
        SortBy::ExpectedGoalsConceded => players.sort_by(|a, b| {
            parse_f64(&b.expected_goals_conceded)
                .partial_cmp(&parse_f64(&a.expected_goals_conceded))
                .unwrap()
        }),
        SortBy::Form => {
            players.sort_by(|a, b| parse_f64(&b.form).partial_cmp(&parse_f64(&a.form)).unwrap())
        }
        SortBy::GoalsConceded => players.sort_by(|a, b| b.goals_conceded.cmp(&a.goals_conceded)),
        SortBy::GoalsScored => players.sort_by(|a, b| b.goals_scored.cmp(&a.goals_scored)),
        SortBy::IctIndex => players.sort_by(|a, b| {
            parse_f64(&b.ict_index)
                .partial_cmp(&parse_f64(&a.ict_index))
                .unwrap()
        }),
        SortBy::Influence => players.sort_by(|a, b| {
            parse_f64(&b.influence)
                .partial_cmp(&parse_f64(&a.influence))
                .unwrap()
        }),
        SortBy::Minutes => players.sort_by(|a, b| b.minutes.cmp(&a.minutes)),
        SortBy::OwnGoals => players.sort_by(|a, b| b.own_goals.cmp(&a.own_goals)),
        SortBy::PenaltiesMissed => {
            players.sort_by(|a, b| b.penalties_missed.cmp(&a.penalties_missed))
        }
        SortBy::PenaltiesSaved => players.sort_by(|a, b| b.penalties_saved.cmp(&a.penalties_saved)),
        SortBy::Points => players.sort_by(|a, b| b.total_points.cmp(&a.total_points)),
        SortBy::PointsPerGame => players.sort_by(|a, b| {
            parse_f64(&b.points_per_game)
                .partial_cmp(&parse_f64(&a.points_per_game))
                .unwrap()
        }),
        SortBy::PriceRiseEvent => {
            players.sort_by(|a, b| b.cost_change_event.cmp(&a.cost_change_event))
        }
        SortBy::PriceRiseStart => {
            players.sort_by(|a, b| b.cost_change_start.cmp(&a.cost_change_start))
        }
        SortBy::Recoveries => players.sort_by(|a, b| b.recoveries.cmp(&a.recoveries)),
        SortBy::RedCards => players.sort_by(|a, b| b.red_cards.cmp(&a.red_cards)),
        SortBy::Saves => players.sort_by(|a, b| b.saves.cmp(&a.saves)),
        SortBy::SelectedBy => players.sort_by(|a, b| {
            parse_f64(&b.selected_by_percent)
                .partial_cmp(&parse_f64(&a.selected_by_percent))
                .unwrap()
        }),
        SortBy::Starts => players.sort_by(|a, b| b.starts.cmp(&a.starts)),
        SortBy::Tackles => players.sort_by(|a, b| b.tackles.cmp(&a.tackles)),
        SortBy::Threat => players.sort_by(|a, b| {
            parse_f64(&b.threat)
                .partial_cmp(&parse_f64(&a.threat))
                .unwrap()
        }),
        SortBy::TransfersIn => players.sort_by(|a, b| b.transfers_in.cmp(&a.transfers_in)),
        SortBy::TransfersInEvent => {
            players.sort_by(|a, b| b.transfers_in_event.cmp(&a.transfers_in_event))
        }
        SortBy::TransfersOut => players.sort_by(|a, b| b.transfers_out.cmp(&a.transfers_out)),
        SortBy::TransfersOutEvent => {
            players.sort_by(|a, b| b.transfers_out_event.cmp(&a.transfers_out_event))
        }
        SortBy::ValueForm => players.sort_by(|a, b| {
            parse_f64(&b.value_form)
                .partial_cmp(&parse_f64(&a.value_form))
                .unwrap()
        }),
        SortBy::ValueSeason => players.sort_by(|a, b| {
            parse_f64(&b.value_season)
                .partial_cmp(&parse_f64(&a.value_season))
                .unwrap()
        }),
        SortBy::YellowCards => players.sort_by(|a, b| b.yellow_cards.cmp(&a.yellow_cards)),
    }
}

fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}

fn print_players(players: &[Element], args: &PlayerFilterArgs, team_map: &HashMap<u64, String>) {
    let stat_label = args.sort.stat_label();

    if let Some(label) = stat_label {
        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>sel_w$}  {:>form_w$}  {:>pts_w$}  {:>avail_w$}  {:>stat_w$}",
            "ID",
            "Name",
            "Pos",
            "Team",
            "Cost",
            "Sel%",
            "Form",
            "Pts",
            "Avail",
            label,
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            cost_w = WIDTH_COST,
            sel_w = WIDTH_SEL,
            form_w = WIDTH_FORM,
            pts_w = WIDTH_PTS,
            avail_w = WIDTH_AVAIL,
            stat_w = WIDTH_STAT,
        );
    } else {
        println!(
            "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>sel_w$}  {:>form_w$}  {:>pts_w$}  {:>avail_w$}  {:<news_w$}",
            "ID",
            "Name",
            "Pos",
            "Team",
            "Cost",
            "Sel%",
            "Form",
            "Pts",
            "Avail",
            "News",
            id_w = WIDTH_ID,
            name_w = WIDTH_NAME,
            pos_w = WIDTH_POS,
            team_w = WIDTH_TEAM_SHORT_NAME,
            cost_w = WIDTH_COST,
            sel_w = WIDTH_SEL,
            form_w = WIDTH_FORM,
            pts_w = WIDTH_PTS,
            avail_w = WIDTH_AVAIL,
            news_w = WIDTH_NEWS,
        );
    }

    for player in players.iter().take(args.limit) {
        let team_name = team_map
            .get(&player.team)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let pos_name = Position::from_element_type_id(player.element_type)
            .map(|p| p.display_name())
            .unwrap_or("N/A");

        let cost = format!("{:.1}", player.now_cost as f64 / 10.0);
        let avail = format_chance_of_playing(player.chance_of_playing_next_round, &player.news);

        if stat_label.is_some() {
            let stat_value = get_stat_value(player, &args.sort);
            println!(
                "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>sel_w$}  {:>form_w$}  {:>pts_w$}  {}  {:>stat_w$}",
                player.id,
                truncate(&player.web_name, WIDTH_NAME),
                pos_name,
                truncate(team_name, WIDTH_TEAM_SHORT_NAME),
                cost,
                player.selected_by_percent,
                player.form,
                player.total_points,
                avail,
                stat_value,
                id_w = WIDTH_ID,
                name_w = WIDTH_NAME,
                pos_w = WIDTH_POS,
                team_w = WIDTH_TEAM_SHORT_NAME,
                cost_w = WIDTH_COST,
                sel_w = WIDTH_SEL,
                form_w = WIDTH_FORM,
                pts_w = WIDTH_PTS,
                stat_w = WIDTH_STAT,
            );
        } else {
            println!(
                "{:>id_w$}  {:<name_w$}  {:<pos_w$}  {:<team_w$}  {:>cost_w$}  {:>sel_w$}  {:>form_w$}  {:>pts_w$}  {}  {:<news_w$}",
                player.id,
                truncate(&player.web_name, WIDTH_NAME),
                pos_name,
                truncate(team_name, WIDTH_TEAM_SHORT_NAME),
                cost,
                player.selected_by_percent,
                player.form,
                player.total_points,
                avail,
                player.news,
                id_w = WIDTH_ID,
                name_w = WIDTH_NAME,
                pos_w = WIDTH_POS,
                team_w = WIDTH_TEAM_SHORT_NAME,
                cost_w = WIDTH_COST,
                sel_w = WIDTH_SEL,
                form_w = WIDTH_FORM,
                pts_w = WIDTH_PTS,
                news_w = WIDTH_NEWS,
            );
        }
    }
}

fn get_stat_value(player: &Element, sort_by: &SortBy) -> String {
    match sort_by {
        SortBy::Assists => player.assists.to_string(),
        SortBy::Bonus => player.bonus.to_string(),
        SortBy::Bps => player.bps.to_string(),
        SortBy::CleanSheets => player.clean_sheets.to_string(),
        SortBy::ClearancesBlocksInterceptions => player.clearances_blocks_interceptions.to_string(),
        SortBy::Creativity => player.creativity.clone(),
        SortBy::DefensiveContribution => player.defensive_contribution.to_string(),
        SortBy::DreamTeamCount => player.dreamteam_count.to_string(),
        SortBy::ExpectedAssists => player.expected_assists.clone(),
        SortBy::ExpectedGoalInvolvements => player.expected_goal_involvements.clone(),
        SortBy::ExpectedGoals => player.expected_goals.clone(),
        SortBy::ExpectedGoalsConceded => player.expected_goals_conceded.clone(),
        SortBy::GoalsConceded => player.goals_conceded.to_string(),
        SortBy::GoalsScored => player.goals_scored.to_string(),
        SortBy::IctIndex => player.ict_index.clone(),
        SortBy::Influence => player.influence.clone(),
        SortBy::Minutes => player.minutes.to_string(),
        SortBy::OwnGoals => player.own_goals.to_string(),
        SortBy::PenaltiesMissed => player.penalties_missed.to_string(),
        SortBy::PenaltiesSaved => player.penalties_saved.to_string(),
        SortBy::PointsPerGame => player.points_per_game.clone(),
        SortBy::PriceRiseEvent => format!("{:.1}", player.cost_change_event as f64 / 10.0),
        SortBy::PriceRiseStart => format!("{:.1}", player.cost_change_start as f64 / 10.0),
        SortBy::Recoveries => player.recoveries.to_string(),
        SortBy::RedCards => player.red_cards.to_string(),
        SortBy::Saves => player.saves.to_string(),
        SortBy::Starts => player.starts.to_string(),
        SortBy::Tackles => player.tackles.to_string(),
        SortBy::Threat => player.threat.clone(),
        SortBy::TransfersIn => player.transfers_in.to_string(),
        SortBy::TransfersInEvent => player.transfers_in_event.to_string(),
        SortBy::TransfersOut => player.transfers_out.to_string(),
        SortBy::TransfersOutEvent => player.transfers_out_event.to_string(),
        SortBy::ValueForm => player.value_form.clone(),
        SortBy::ValueSeason => player.value_season.clone(),
        SortBy::YellowCards => player.yellow_cards.to_string(),
        _ => "".to_string(),
    }
}
