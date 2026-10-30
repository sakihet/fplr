use crate::api::FplClient;
use crate::error::{FplrError, Result};
use crate::models::ManagerPicks;

/// Fetch a manager's picks for a gameweek, falling back to the previous one
/// while the requested gameweek's squad is not public yet
pub async fn fetch_picks_with_fallback(manager_id: u64, event_id: u32) -> Result<ManagerPicks> {
    match FplClient::fetch_manager_picks(manager_id, event_id).await {
        Ok(data) => Ok(data),
        Err(FplrError::ApiStatus(status, _)) if status.as_u16() == 404 && event_id > 1 => {
            FplClient::fetch_manager_picks(manager_id, event_id - 1).await
        }
        Err(e) => Err(e),
    }
}
