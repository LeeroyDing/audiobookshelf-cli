mod client;
mod commands;
mod models;

use anyhow::{Context, Result};
use clap::Parser;
use client::AbsClient;
use commands::{handle_command, Cli};
use directories::ProjectDirs;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Config {
    server_url: Option<String>,
    api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load variables from .env if it exists
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    // Setup config loading
    let mut builder = config::Config::builder();

    // 1. Load from global config file
    if let Some(proj_dirs) = ProjectDirs::from("com", "audiobookshelf", "abs") {
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.yaml");
        if config_path.exists() {
            builder = builder.add_source(config::File::from(config_path));
        }
    }

    // 2. Load from environment variables (overrides file)
    // Map AUDIOBOOKSHELF_SERVER_URL to server_url, etc.
    builder = builder.add_source(
        config::Environment::with_prefix("AUDIOBOOKSHELF")
            .separator("_")
            .try_parsing(true),
    );

    let settings = builder.build()?;
    let config: Config = settings.try_deserialize()?;

    // Priority: Env Var > .env > Config File
    // Note: dotenvy already populated env vars, and config::Environment picked them up.
    
    let server_url = config.server_url
        .or_else(|| env::var("AUDIOBOOKSHELF_SERVER_URL").ok())
        .context("Server URL not found. Set AUDIOBOOKSHELF_SERVER_URL in environment, .env, or config.yaml")?;

    let api_key = config.api_key
        .or_else(|| env::var("AUDIOBOOKSHELF_API_KEY").ok())
        .context("API Key not found. Set AUDIOBOOKSHELF_API_KEY in environment, .env, or config.yaml")?;

    let client = AbsClient::new(server_url, api_key);

    handle_command(cli, client).await?;

    Ok(())
}
