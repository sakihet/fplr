use serde::de::DeserializeOwned;

use crate::models::{BootstrapStatic, DreamTeam, Fixture, LiveData, ManagerHistory, ManagerPicks, PlayerSummary};

const BASE_URL: &str = "https://fantasy.premierleague.com/api";

pub struct FplClient;

impl FplClient {
    async fn fetch<T: DeserializeOwned>(endpoint: &str) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = reqwest::get(&url).await?;
        let json: T = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_bootstrap_static() -> Result<BootstrapStatic, Box<dyn std::error::Error>> {
        Self::fetch("/bootstrap-static/").await
    }

    pub async fn fetch_dream_team(event_id: u32) -> Result<DreamTeam, Box<dyn std::error::Error>> {
        Self::fetch(&format!("/dream-team/{}/", event_id)).await
    }

    pub async fn fetch_fixtures() -> Result<Vec<Fixture>, Box<dyn std::error::Error>> {
        Self::fetch("/fixtures/").await
    }

    pub async fn fetch_live(event_id: u32) -> Result<LiveData, Box<dyn std::error::Error>> {
        Self::fetch(&format!("/event/{}/live/", event_id)).await
    }

    pub async fn fetch_manager_picks(
        manager_id: u64,
        event_id: u32,
    ) -> Result<ManagerPicks, Box<dyn std::error::Error>> {
        Self::fetch(&format!("/entry/{}/event/{}/picks/", manager_id, event_id)).await
    }

    pub async fn fetch_player_summary(
        player_id: u64,
    ) -> Result<PlayerSummary, Box<dyn std::error::Error>> {
        Self::fetch(&format!("/element-summary/{}/", player_id)).await
    }

    pub async fn fetch_manager_history(
        manager_id: u64,
    ) -> Result<ManagerHistory, Box<dyn std::error::Error>> {
        Self::fetch(&format!("/entry/{}/history/", manager_id)).await
    }
}
