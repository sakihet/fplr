use thiserror::Error;

#[derive(Error, Debug)]
pub enum FplrError {
    #[error("API request failed: {0}")]
    Api(#[from] reqwest::Error),

    #[error("API request failed with status {0}: {1}")]
    ApiStatus(reqwest::StatusCode, String),

    #[error("Failed to parse API response: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Failed to read config file: {0}")]
    ConfigRead(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("Failed to serialize config: {0}")]
    ConfigSerialize(#[from] toml::ser::Error),

    #[error("Could not determine config path")]
    ConfigPathNotFound,

    #[error("Manager ID not configured. Run `fplr config set manager-id <ID>` to set it.")]
    ManagerIdNotSet,

    #[error("Invalid manager ID: {0}")]
    InvalidManagerId(String),

    #[error("No next event found")]
    NoNextEvent,

    #[error("No previous event found")]
    NoPreviousEvent,

    #[error("Team not found: {0}")]
    TeamNotFound(u64),

    #[error("Team not found: {0}")]
    TeamNotFoundByName(String),

    #[error("Player not found: {0}")]
    PlayerNotFound(u64),

    #[error("Unknown config key: {0}")]
    UnknownConfigKey(String),

    #[error("Fixture not found: {0}")]
    FixtureNotFound(u64),

    #[error("Fixture is not scheduled (no Gameweek assigned): {0}")]
    FixtureNotScheduled(u64),
}

pub type Result<T> = std::result::Result<T, FplrError>;
