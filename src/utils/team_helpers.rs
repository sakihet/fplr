use std::collections::HashMap;

use crate::models::Team;

pub fn create_team_map(teams: &[Team]) -> HashMap<u64, String> {
    teams
        .iter()
        .map(|team| (team.id, team.name.clone()))
        .collect()
}

pub fn find_team_ids_by_name(teams: &[Team], name: &str) -> Vec<u64> {
    let search_term = name.to_lowercase();
    teams
        .iter()
        .filter(|team| {
            team.name.to_lowercase().contains(&search_term)
                || team.short_name.to_lowercase().contains(&search_term)
        })
        .map(|team| team.id)
        .collect()
}
