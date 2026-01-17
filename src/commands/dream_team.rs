use crate::api::FplClient;
use crate::utils::player_helpers::create_player_map;

pub async fn handle_dream_team(event_id: u32) {
    match FplClient::fetch_bootstrap_static().await {
        Ok(bootstrap_data) => {
            let player_map = create_player_map(&bootstrap_data.elements);

            match FplClient::fetch_dream_team(event_id).await {
                Ok(data) => {
                    let mut team = data.team;
                    team.sort_by(|a, b| b.points.cmp(&a.points));

                    println!("{:<4} {:<20} {:<12}", "ID", "Name", "Points");
                    for t in team.iter() {
                        let name = player_map
                            .get(&t.element)
                            .map(|s| s.as_str())
                            .unwrap_or("Unknown");

                        println!("{:<4} {:<20} {:<12}", t.element, name, t.points);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
