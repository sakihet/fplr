use std::collections::HashMap;

use crate::models::Team;

/// Create a map of team ID to team full name
pub fn create_team_map(teams: &[Team]) -> HashMap<u64, String> {
    teams
        .iter()
        .map(|team| (team.id, team.name.clone()))
        .collect()
}

/// Create a map of team ID to team short name (e.g., "ARS", "CHE")
pub fn create_team_short_name_map(teams: &[Team]) -> HashMap<u64, String> {
    teams
        .iter()
        .map(|team| (team.id, team.short_name.clone()))
        .collect()
}

/// Create a map of team ID to team reference
pub fn create_team_ref_map(teams: &[Team]) -> HashMap<u64, &Team> {
    teams.iter().map(|team| (team.id, team)).collect()
}

/// Find team IDs by name (searches both full name and short name)
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
