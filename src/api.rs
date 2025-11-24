use serde_json::Value;

use crate::models::BootstrapStatic;

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
}
