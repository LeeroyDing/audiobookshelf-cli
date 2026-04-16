use clap::{Subcommand, Parser};
use crate::client::AbsClient;
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check if the server is reachable and authentication works
    Ping,
    /// Library-related operations
    Libraries {
        #[command(subcommand)]
        cmd: LibraryCommands,
    },
    /// User-related operations
    Users {
        #[command(subcommand)]
        cmd: UserCommands,
    },
    /// Item-related operations
    Items {
        #[command(subcommand)]
        cmd: ItemCommands,
    },
    /// Author-related operations
    Authors {
        #[command(subcommand)]
        cmd: AuthorCommands,
    },
    /// Collection-related operations
    Collections {
        #[command(subcommand)]
        cmd: CollectionCommands,
    },
    /// Playlist-related operations
    Playlists {
        #[command(subcommand)]
        cmd: PlaylistCommands,
    },
    /// Series-related operations
    Series {
        #[command(subcommand)]
        cmd: SeriesCommands,
    },
    /// Metadata-related operations (tags, genres)
    Metadata {
        #[command(subcommand)]
        cmd: MetadataCommands,
    },
    /// Get details about the current user
    Me,
}

#[derive(Subcommand)]
pub enum LibraryCommands {
    /// List all libraries
    List,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List all system users
    List,
}

#[derive(Subcommand)]
pub enum ItemCommands {
    /// List all items in a specific library
    List {
        /// The ID of the library
        library_id: String,
    },
    /// Get details about a specific item
    Get {
        /// The ID of the item
        item_id: String,
    },
}

#[derive(Subcommand)]
pub enum AuthorCommands {
    /// List all authors
    List,
    /// Get details about a specific author
    Get {
        /// The ID of the author
        id: String,
    },
}

#[derive(Subcommand)]
pub enum CollectionCommands {
    /// List all collections
    List,
    /// Get details about a specific collection
    Get {
        /// The ID of the collection
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PlaylistCommands {
    /// List all playlists
    List,
    /// Get details about a specific playlist
    Get {
        /// The ID of the playlist
        id: String,
    },
}

#[derive(Subcommand)]
pub enum SeriesCommands {
    /// List all series
    List,
    /// Get details about a specific series
    Get {
        /// The ID of the series
        id: String,
    },
}

#[derive(Subcommand)]
pub enum MetadataCommands {
    /// List all tags
    Tags,
    /// List all genres
    Genres,
}

pub async fn handle_command(cli: Cli, client: AbsClient) -> Result<()> {
    match cli.command {
        Commands::Ping => {
            println!("Pinging server...");
            client.ping().await?;
            println!("Server is reachable!");
        }
        Commands::Libraries { cmd } => match cmd {
            LibraryCommands::List => {
                let libs = client.get_libraries().await?;
                // Parse it out using models if we want, or just pretty print json
                // For simplicity let's pretty print the "libraries" array if it exists
                if let Some(libraries) = libs.get("libraries") {
                    println!("{}", serde_json::to_string_pretty(libraries)?);
                } else {
                    println!("{}", serde_json::to_string_pretty(&libs)?);
                }
            }
        },
        Commands::Users { cmd } => match cmd {
            UserCommands::List => {
                let users = client.get_users().await?;
                // users request returns an array directly, according to usual audiobookshelf API
                println!("{}", serde_json::to_string_pretty(&users)?);
            }
        },
        Commands::Items { cmd } => match cmd {
            ItemCommands::List { library_id } => {
                let items = client.get_library_items(&library_id).await?;
                println!("{}", serde_json::to_string_pretty(&items)?);
            }
            ItemCommands::Get { item_id } => {
                let item = client.get_item(&item_id).await?;
                println!("{}", serde_json::to_string_pretty(&item)?);
            }
        },
        Commands::Authors { cmd } => match cmd {
            AuthorCommands::List => {
                let authors = client.get_authors().await?;
                println!("{}", serde_json::to_string_pretty(&authors)?);
            }
            AuthorCommands::Get { id } => {
                let author = client.get_author(&id).await?;
                println!("{}", serde_json::to_string_pretty(&author)?);
            }
        },
        Commands::Collections { cmd } => match cmd {
            CollectionCommands::List => {
                let collections = client.get_collections().await?;
                println!("{}", serde_json::to_string_pretty(&collections)?);
            }
            CollectionCommands::Get { id } => {
                let collection = client.get_collection(&id).await?;
                println!("{}", serde_json::to_string_pretty(&collection)?);
            }
        },
        Commands::Playlists { cmd } => match cmd {
            PlaylistCommands::List => {
                let playlists = client.get_playlists().await?;
                println!("{}", serde_json::to_string_pretty(&playlists)?);
            }
            PlaylistCommands::Get { id } => {
                let playlist = client.get_playlist(&id).await?;
                println!("{}", serde_json::to_string_pretty(&playlist)?);
            }
        },
        Commands::Series { cmd } => match cmd {
            SeriesCommands::List => {
                let series = client.get_series_list().await?;
                println!("{}", serde_json::to_string_pretty(&series)?);
            }
            SeriesCommands::Get { id } => {
                let series = client.get_series(&id).await?;
                println!("{}", serde_json::to_string_pretty(&series)?);
            }
        },
        Commands::Metadata { cmd } => match cmd {
            MetadataCommands::Tags => {
                let tags = client.get_tags().await?;
                println!("{}", serde_json::to_string_pretty(&tags)?);
            }
            MetadataCommands::Genres => {
                let genres = client.get_genres().await?;
                println!("{}", serde_json::to_string_pretty(&genres)?);
            }
        },
        Commands::Me => {
            let me = client.get_me().await?;
            println!("{}", serde_json::to_string_pretty(&me)?);
        }
    }
    
    Ok(())
}
