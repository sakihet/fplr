use std::fs;

use crate::cache;
use crate::error::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    /// Remove all cached responses
    Clear,
    /// Show cache location, entry count and total size
    Info,
}

pub fn handle_cache(args: CacheArgs) -> Result<()> {
    let Some(dir) = cache::cache_dir() else {
        println!("Could not determine cache directory.");
        return Ok(());
    };

    match args.command {
        CacheCommands::Clear => {
            if !dir.exists() {
                println!("Cache is already empty.");
                return Ok(());
            }
            match fs::remove_dir_all(&dir) {
                Ok(()) => println!("Cleared cache at {}", dir.display()),
                Err(e) => eprintln!("Failed to clear cache: {}", e),
            }
        }
        CacheCommands::Info => {
            println!("Cache directory: {}", dir.display());

            let mut count = 0;
            let mut bytes = 0;
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata()
                        && metadata.is_file()
                    {
                        count += 1;
                        bytes += metadata.len();
                    }
                }
            }
            println!("Entries: {}", count);
            println!("Size: {:.2} MB", bytes as f64 / 1_048_576.0);
        }
    }
    Ok(())
}
