use serde_json::Value;

use crate::models::{BootstrapStatic, LiveData, PlayerSummary};

pub struct FplClient;

impl FplClient {
    pub async fn fetch_bootstrap_static() -> Result<BootstrapStatic, Box<dyn std::error::Error>> {
        let url = "https://fantasy.premierleague.com/api/bootstrap-static/";
        let response = reqwest::get(url).await?;
        let json: BootstrapStatic = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_fixtures() -> Result<Value, Box<dyn std::error::Error>> {
        let url = "https://fantasy.premierleague.com/api/fixtures/";
        let response = reqwest::get(url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_live(event_id: u32) -> Result<LiveData, Box<dyn std::error::Error>> {
        let url = format!(
            "https://fantasy.premierleague.com/api/event/{}/live/",
            event_id
        );
        let response = reqwest::get(url).await?;
        let json: LiveData = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_player_summary(
        player_id: u64,
    ) -> Result<PlayerSummary, Box<dyn std::error::Error>> {
        let url = format!(
            "https://fantasy.premierleague.com/api/element-summary/{}/",
            player_id
        );
        let response = reqwest::get(url).await?;
        let json: PlayerSummary = response.json().await?;
        Ok(json)
    }
}
