use std::sync::OnceLock;
use std::time::Duration;

use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::models::{
    BootstrapStatic, DreamTeam, EntryDetail, Fixture, LeagueStandingsResponse, LiveData,
    ManagerHistory, ManagerPicks, PlayerSummary, SetPieceNotes, Transfer,
};

const BASE_URL: &str = "https://fantasy.premierleague.com/api";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build HTTP client")
    })
}

pub struct FplClient;

impl FplClient {
    async fn fetch<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = http_client().get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "".to_string());
            return Err(crate::error::FplrError::ApiStatus(status, body));
        }

        let json: T = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_bootstrap_static() -> Result<BootstrapStatic> {
        Self::fetch("/bootstrap-static/").await
    }

    pub async fn fetch_dream_team(event_id: u32) -> Result<DreamTeam> {
        Self::fetch(&format!("/dream-team/{}/", event_id)).await
    }

    pub async fn fetch_fixtures() -> Result<Vec<Fixture>> {
        Self::fetch("/fixtures/").await
    }

    pub async fn fetch_fixtures_by_event(event_id: u32) -> Result<Vec<Fixture>> {
        Self::fetch(&format!("/fixtures/?event={}", event_id)).await
    }

    pub async fn fetch_live(event_id: u32) -> Result<LiveData> {
        Self::fetch(&format!("/event/{}/live/", event_id)).await
    }

    pub async fn fetch_manager_picks(manager_id: u64, event_id: u32) -> Result<ManagerPicks> {
        Self::fetch(&format!("/entry/{}/event/{}/picks/", manager_id, event_id)).await
    }

    pub async fn fetch_player_summary(player_id: u64) -> Result<PlayerSummary> {
        Self::fetch(&format!("/element-summary/{}/", player_id)).await
    }

    pub async fn fetch_manager_history(manager_id: u64) -> Result<ManagerHistory> {
        Self::fetch(&format!("/entry/{}/history/", manager_id)).await
    }

    pub async fn fetch_set_piece_notes() -> Result<SetPieceNotes> {
        Self::fetch("/team/set-piece-notes/").await
    }

    pub async fn fetch_league_standings(league_id: u32) -> Result<LeagueStandingsResponse> {
        Self::fetch(&format!("/leagues-classic/{}/standings/", league_id)).await
    }

    pub async fn fetch_manager_transfers(manager_id: u64) -> Result<Vec<Transfer>> {
        Self::fetch(&format!("/entry/{}/transfers/", manager_id)).await
    }

    pub async fn fetch_entry_details(manager_id: u64) -> Result<EntryDetail> {
        Self::fetch(&format!("/entry/{}/", manager_id)).await
    }
}
