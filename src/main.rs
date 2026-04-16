mod client;
mod commands;
mod models;

use anyhow::{Context, Result};
use clap::Parser;
use client::AbsClient;
use commands::{handle_command, Cli};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load variables from .env if it exists
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    let server_url = env::var("AUDIOBOOKSHELF_SERVER_URL")
        .context("AUDIOBOOKSHELF_SERVER_URL is not set. Please set it in your environment or .env file.")?;
    
    let api_key = env::var("AUDIOBOOKSHELF_API_KEY")
        .context("AUDIOBOOKSHELF_API_KEY is not set. Please set it in your environment or .env file.")?;

    let client = AbsClient::new(server_url, api_key);

    handle_command(cli, client).await?;

    Ok(())
}
