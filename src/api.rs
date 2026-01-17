use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::models::{BootstrapStatic, DreamTeam, Fixture, LiveData, ManagerPicks, PlayerSummary};

pub struct FplClient;

impl FplClient {
    async fn fetch_json<T: DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let json: T = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_bootstrap_static() -> Result<BootstrapStatic, Box<dyn std::error::Error>> {
        Self::fetch_json("https://fantasy.premierleague.com/api/bootstrap-static/").await
    }

    pub async fn fetch_dream_team(event_id: u32) -> Result<DreamTeam, Box<dyn std::error::Error>> {
        let url = format!("https://fantasy.premierleague.com/api/dream-team/{}/", event_id);
        Self::fetch_json(&url).await
    }

    pub async fn fetch_fixtures() -> Result<Value, Box<dyn std::error::Error>> {
        Self::fetch_json("https://fantasy.premierleague.com/api/fixtures/").await
    }

    pub async fn fetch_fixtures_typed() -> Result<Vec<Fixture>, Box<dyn std::error::Error>> {
        Self::fetch_json("https://fantasy.premierleague.com/api/fixtures/").await
    }

    pub async fn fetch_live(event_id: u32) -> Result<LiveData, Box<dyn std::error::Error>> {
        let url = format!("https://fantasy.premierleague.com/api/event/{}/live/", event_id);
        Self::fetch_json(&url).await
    }

    pub async fn fetch_manager_picks(
        manager_id: u64,
        event_id: u32,
    ) -> Result<ManagerPicks, Box<dyn std::error::Error>> {
        let url = format!(
            "https://fantasy.premierleague.com/api/entry/{}/event/{}/picks/",
            manager_id, event_id
        );
        Self::fetch_json(&url).await
    }

    pub async fn fetch_player_summary(
        player_id: u64,
    ) -> Result<PlayerSummary, Box<dyn std::error::Error>> {
        let url = format!(
            "https://fantasy.premierleague.com/api/element-summary/{}/",
            player_id
        );
        Self::fetch_json(&url).await
    }
}
