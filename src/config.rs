use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::error::{FplrError, Result};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub manager_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub user: Option<UserConfig>,
}

impl Config {
    pub fn get_config_path() -> Option<PathBuf> {
        let mut path = dirs::home_dir()?;
        path.push(".config");
        path.push("fplr");
        path.push("config.toml");
        Some(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_config_path().ok_or(FplrError::ConfigPathNotFound)?;

        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path().ok_or(FplrError::ConfigPathNotFound)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Get manager ID from config, returning an error if not set
    pub fn get_manager_id(&self) -> Result<u64> {
        let id_str = self
            .user
            .as_ref()
            .and_then(|u| u.manager_id.as_ref())
            .ok_or(FplrError::ManagerIdNotSet)?;

        id_str
            .parse()
            .map_err(|_| FplrError::InvalidManagerId(id_str.clone()))
    }
}
