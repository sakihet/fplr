use crate::config::Config;
use crate::error::{FplrError, Result};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Set a configuration value
    Set {
        /// The configuration key (e.g., manager-id)
        key: String,
        /// The value to set
        value: String,
    },
    /// Get a configuration value
    Get {
        /// The configuration key
        key: String,
    },
    /// List all configuration values
    List,
}

pub fn handle_config(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommands::Set { key, value } => {
            let mut config = Config::load().unwrap_or_default();

            match key.as_str() {
                "manager-id" => {
                    let mut user = config.user.unwrap_or_default();
                    user.manager_id = Some(value.clone());
                    config.user = Some(user);
                    config.save()?;
                    println!("Successfully updated manager-id to {}", value);
                }
                _ => {
                    return Err(FplrError::UnknownConfigKey(key));
                }
            }
        }
        ConfigCommands::Get { key } => {
            let config = Config::load().unwrap_or_default();
            match key.as_str() {
                "manager-id" => {
                    if let Some(user) = config.user {
                        if let Some(id) = user.manager_id {
                            println!("{}", id);
                        } else {
                            println!("manager-id is not set");
                        }
                    } else {
                        println!("manager-id is not set");
                    }
                }
                _ => {
                    return Err(FplrError::UnknownConfigKey(key));
                }
            }
        }
        ConfigCommands::List => {
            let config = Config::load().unwrap_or_default();
            if let Some(path) = Config::get_config_path() {
                println!("Config file: {}", path.display());
            }
            if let Some(user) = config.user {
                if let Some(id) = user.manager_id {
                    println!("manager-id = {}", id);
                }
                if let Some(name) = user.name {
                    println!("name = {}", name);
                }
            } else {
                println!("No configuration set.");
            }
        }
    }
    Ok(())
}
