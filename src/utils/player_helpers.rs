use std::collections::HashMap;

use crate::models::Element;

pub fn create_player_map(elements: &[Element]) -> HashMap<u64, String> {
    elements
        .iter()
        .map(|player| (player.id, player.web_name.clone()))
        .collect()
}
