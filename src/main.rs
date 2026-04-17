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

    // 3. Handle Auth command early if possible (doesn't need server_url/api_key resolution)
    // Actually, it's easier to just resolve them if needed.

    let server_url = config
        .server_url
        .or_else(|| env::var("AUDIOBOOKSHELF_SERVER_URL").ok())
        .unwrap_or_default(); // We'll context check this later if needed

    let mut api_key = config
        .api_key
        .or_else(|| env::var("AUDIOBOOKSHELF_API_KEY").ok());

    // 4. Try Keyring if still missing
    if api_key.is_none() {
        if let Ok(entry) = keyring::Entry::new("audiobookshelf-cli", "api_key") {
            if let Ok(pw) = entry.get_password() {
                api_key = Some(pw);
            }
        }
    }

    // Special case: Auth commands don't strictly need a valid client setup to run login/logout
    if let commands::Commands::Auth { .. } = &cli.command {
        let client = AbsClient::new(server_url, api_key.unwrap_or_default());
        handle_command(cli, client).await?;
        return Ok(());
    }

    // For all other commands, require both
    if server_url.is_empty() {
        anyhow::bail!(
            "Server URL not found. Set AUDIOBOOKSHELF_SERVER_URL in environment or config.yaml"
        );
    }
    let key = api_key.context("API Key not found. Please run 'abs auth login --api-key <KEY>' or set AUDIOBOOKSHELF_API_KEY")?;

    let client = AbsClient::new(server_url, key);

    handle_command(cli, client).await?;

    Ok(())
}
