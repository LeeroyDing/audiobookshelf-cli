use crate::client::AbsClient;
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Output raw JSON instead of human-readable tables. Useful for scripting and AI agents.
    #[arg(short, long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check server connectivity and API key validity
    Ping,
    /// Manage libraries (list, scan)
    Libraries {
        #[command(subcommand)]
        cmd: LibraryCommands,
    },
    /// Manage system users
    Users {
        #[command(subcommand)]
        cmd: UserCommands,
    },
    /// Manage library items (books, etc.)
    Items {
        #[command(subcommand)]
        cmd: ItemCommands,
    },
    /// Manage authors
    Authors {
        #[command(subcommand)]
        cmd: AuthorCommands,
    },
    /// Manage collections
    Collections {
        #[command(subcommand)]
        cmd: CollectionCommands,
    },
    /// Manage playlists
    Playlists {
        #[command(subcommand)]
        cmd: PlaylistCommands,
    },
    /// Manage series
    Series {
        #[command(subcommand)]
        cmd: SeriesCommands,
    },
    /// Explore metadata (tags, genres)
    Metadata {
        #[command(subcommand)]
        cmd: MetadataCommands,
    },
    /// Get details about the authenticated user
    Me,
    /// Get server status, version, and filesystem paths
    Info,
    /// Global search for books, authors, series, and more across all libraries
    Search {
        /// The search query (e.g., "Andy Weir")
        query: String,
    },
    /// Upload audiobooks or eBooks to a library
    Upload {
        /// Local file paths to upload
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
        /// The UUID of the library to upload to
        #[arg(short, long)]
        library: String,
        /// The UUID of the folder within the library. If omitted, will auto-resolve if the library has only one folder.
        #[arg(short, long)]
        folder: Option<String>,
        /// The title of the book
        #[arg(short, long)]
        title: String,
        /// The author's name
        #[arg(short, long)]
        author: Option<String>,
        /// The series name
        #[arg(short, long)]
        series: Option<String>,
    },
    /// Manage authentication (login/logout)
    Auth {
        #[command(subcommand)]
        cmd: AuthCommands,
    },
    /// Generate shell completion scripts (Bash, Zsh, Fish, PowerShell)
    Completion {
        /// The shell to generate completions for
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
pub enum LibraryCommands {
    /// List all libraries on the server
    List,
    /// Trigger a scan for new files in a specific library
    Scan {
        /// The UUID of the library to scan
        id: String,
        /// Force a full rescan instead of an incremental one
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List all system users (requires admin permissions)
    List,
}

#[derive(Subcommand)]
pub enum ItemCommands {
    /// List all items in a specific library
    List {
        /// The UUID of the library
        library_id: String,
    },
    /// Get full metadata for a specific item (outputs JSON)
    Get {
        /// The UUID of the item
        item_id: String,
    },
    /// Update metadata for a specific item
    Update {
        /// The UUID of the item to update
        item_id: String,
        /// Update the title
        #[arg(long)]
        title: Option<String>,
        /// Update the subtitle
        #[arg(long)]
        subtitle: Option<String>,
        /// Update author(s). Separate multiple authors with commas.
        #[arg(long)]
        author: Option<String>,
        /// Update narrator(s). Separate multiple narrators with commas.
        #[arg(long)]
        narrator: Option<String>,
        /// Update the series name
        #[arg(long)]
        series: Option<String>,
        /// Update genres. Separate multiple genres with commas.
        #[arg(long)]
        genres: Option<String>,
        /// Update tags. Separate multiple tags with commas.
        #[arg(long)]
        tags: Option<String>,
        /// Update the published year
        #[arg(long)]
        year: Option<i32>,
    },
    /// Quick match an item against online metadata providers
    Match {
        /// The UUID of the item to match
        item_id: String,
    },
    /// Remove the metadata match from an item (returns it to unmatched state)
    Unmatch {
        /// The UUID of the item to unmatch
        item_id: String,
    },
    /// Update metadata for multiple items at once
    BulkUpdate {
        /// Comma-separated list of item UUIDs (e.g., "id1,id2,id3")
        ids: String,
        /// Update the title for all items
        #[arg(long)]
        title: Option<String>,
        /// Update the subtitle for all items
        #[arg(long)]
        subtitle: Option<String>,
        /// Update author(s). Separate multiple authors with commas.
        #[arg(long)]
        author: Option<String>,
        /// Update narrator(s). Separate multiple narrators with commas.
        #[arg(long)]
        narrator: Option<String>,
        /// Update the series name for all items
        #[arg(long)]
        series: Option<String>,
        /// Update genres. Separate multiple genres with commas.
        #[arg(long)]
        genres: Option<String>,
        /// Update tags. Separate multiple tags with commas.
        #[arg(long)]
        tags: Option<String>,
        /// Update the published year for all items
        #[arg(long)]
        year: Option<i32>,
    },
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Save your API key securely in the system keyring. Subsequent commands will use this key.
    Login {
        /// The API key to save
        #[arg(short, long)]
        api_key: String,
    },
    /// Remove your API key from the system keyring
    Logout,
}

#[derive(Subcommand)]
pub enum AuthorCommands {
    /// List all authors on the server
    List,
    /// Get detailed information about a specific author
    Get {
        /// The UUID of the author
        id: String,
    },
}

#[derive(Subcommand)]
pub enum CollectionCommands {
    /// List all collections on the server
    List,
    /// Get detailed information about a specific collection
    Get {
        /// The UUID of the collection
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PlaylistCommands {
    /// List all playlists on the server
    List,
    /// Get detailed information about a specific playlist
    Get {
        /// The UUID of the playlist
        id: String,
    },
}

#[derive(Subcommand)]
pub enum SeriesCommands {
    /// List all series on the server
    List,
    /// Get detailed information about a specific series
    Get {
        /// The UUID of the series
        id: String,
    },
}

#[derive(Subcommand)]
pub enum MetadataCommands {
    /// List all unique tags used across the server
    Tags,
    /// List all unique genres used across the server
    Genres,
}

use comfy_table::Table;

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
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&libs)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Name", "Media Type"]);

                    let libraries = if let Some(l) = libs.get("libraries") {
                        l.as_array().cloned().unwrap_or_default()
                    } else if libs.is_array() {
                        libs.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![libs]
                    };

                    for lib in libraries {
                        table.add_row(vec![
                            lib.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            lib.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            lib.get("mediaType")
                                .and_then(|v| v.as_str())
                                .unwrap_or("N/A"),
                        ]);
                    }
                    println!("{}", table);
                }
            }
            LibraryCommands::Scan { id, force } => {
                println!("Triggering scan for library {} (force={})...", id, force);
                client.scan_library(&id, force).await?;
                println!("Scan triggered successfully!");
            }
        },
        Commands::Users { cmd } => match cmd {
            UserCommands::List => {
                let users_resp = client.get_users().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&users_resp)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Username", "Type", "Active"]);

                    let users = if let Some(u) = users_resp.get("users") {
                        u.as_array().cloned().unwrap_or_default()
                    } else if users_resp.is_array() {
                        users_resp.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![users_resp]
                    };

                    for user in users {
                        table.add_row(vec![
                            user.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            user.get("username")
                                .and_then(|v| v.as_str())
                                .unwrap_or("N/A"),
                            user.get("type").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            &user
                                .get("isActive")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false)
                                .to_string(),
                        ]);
                    }
                    println!("{}", table);
                }
            }
        },
        Commands::Items { cmd } => match cmd {
            ItemCommands::List { library_id } => {
                let items_resp = client.get_library_items(&library_id).await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&items_resp)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Title", "Media Type"]);

                    let items = if let Some(results) = items_resp.get("results") {
                        results.as_array().cloned().unwrap_or_default()
                    } else if items_resp.is_array() {
                        items_resp.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![items_resp]
                    };

                    for item in items {
                        let title = item
                            .get("media")
                            .and_then(|m| m.get("metadata"))
                            .and_then(|meta| meta.get("title"))
                            .and_then(|t| t.as_str())
                            .or_else(|| {
                                item.get("media")
                                    .and_then(|m| m.get("metadata"))
                                    .and_then(|meta| meta.get("name"))
                                    .and_then(|n| n.as_str())
                            })
                            .unwrap_or("N/A");

                        table.add_row(vec![
                            item.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            title,
                            item.get("mediaType")
                                .and_then(|v| v.as_str())
                                .unwrap_or("N/A"),
                        ]);
                    }
                    println!("{}", table);
                }
            }
            ItemCommands::Get { item_id } => {
                let item = client.get_item(&item_id).await?;
                println!("{}", serde_json::to_string_pretty(&item)?);
            }
            ItemCommands::Update {
                item_id,
                title,
                subtitle,
                author,
                narrator,
                series,
                genres,
                tags,
                year,
            } => {
                let mut meta = serde_json::Map::new();
                if let Some(t) = title {
                    meta.insert("title".to_string(), serde_json::Value::String(t));
                }
                if let Some(s) = subtitle {
                    meta.insert("subtitle".to_string(), serde_json::Value::String(s));
                }
                if let Some(a) = author {
                    meta.insert("authorName".to_string(), serde_json::Value::String(a));
                } // Note: ABS uses authorName for simple updates usually
                if let Some(n) = narrator {
                    meta.insert("narratorName".to_string(), serde_json::Value::String(n));
                }
                if let Some(s) = series {
                    meta.insert("seriesName".to_string(), serde_json::Value::String(s));
                }
                if let Some(y) = year {
                    meta.insert(
                        "publishedYear".to_string(),
                        serde_json::Value::Number(y.into()),
                    );
                }

                if let Some(g_str) = genres {
                    let g_list: Vec<serde_json::Value> = g_str
                        .split(',')
                        .map(|s| serde_json::Value::String(s.trim().to_string()))
                        .collect();
                    meta.insert("genres".to_string(), serde_json::Value::Array(g_list));
                }

                if let Some(t_str) = tags {
                    let t_list: Vec<serde_json::Value> = t_str
                        .split(',')
                        .map(|s| serde_json::Value::String(s.trim().to_string()))
                        .collect();
                    meta.insert("tags".to_string(), serde_json::Value::Array(t_list));
                }

                if meta.is_empty() {
                    anyhow::bail!("No metadata fields provided for update.");
                }

                println!("Updating metadata for item {}...", item_id);
                let result = client
                    .update_item_metadata(&item_id, serde_json::Value::Object(meta))
                    .await?;
                println!("Item updated successfully!");
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
            }
            ItemCommands::Match { item_id } => {
                println!("Matching item {}...", item_id);
                client.match_item(&item_id).await?;
                println!("Matching triggered successfully!");
            }
            ItemCommands::Unmatch { item_id } => {
                println!("Unmatching item {}...", item_id);
                client.unmatch_item(&item_id).await?;
                println!("Item unmatched successfully!");
            }
            ItemCommands::BulkUpdate {
                ids,
                title,
                subtitle,
                author,
                narrator,
                series,
                genres,
                tags,
                year,
            } => {
                let id_list: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();

                let mut meta = serde_json::Map::new();
                if let Some(t) = title {
                    meta.insert("title".to_string(), serde_json::Value::String(t));
                }
                if let Some(s) = subtitle {
                    meta.insert("subtitle".to_string(), serde_json::Value::String(s));
                }
                if let Some(a) = author {
                    meta.insert("authorName".to_string(), serde_json::Value::String(a));
                }
                if let Some(n) = narrator {
                    meta.insert("narratorName".to_string(), serde_json::Value::String(n));
                }
                if let Some(s) = series {
                    meta.insert("seriesName".to_string(), serde_json::Value::String(s));
                }
                if let Some(y) = year {
                    meta.insert(
                        "publishedYear".to_string(),
                        serde_json::Value::Number(y.into()),
                    );
                }

                if let Some(g_str) = genres {
                    let g_list: Vec<serde_json::Value> = g_str
                        .split(',')
                        .map(|s| serde_json::Value::String(s.trim().to_string()))
                        .collect();
                    meta.insert("genres".to_string(), serde_json::Value::Array(g_list));
                }

                if let Some(t_str) = tags {
                    let t_list: Vec<serde_json::Value> = t_str
                        .split(',')
                        .map(|s| serde_json::Value::String(s.trim().to_string()))
                        .collect();
                    meta.insert("tags".to_string(), serde_json::Value::Array(t_list));
                }

                if meta.is_empty() {
                    anyhow::bail!("No metadata fields provided for update.");
                }

                println!("Performing bulk update for {} items...", id_list.len());
                let result = client
                    .batch_update_items(&id_list, serde_json::Value::Object(meta))
                    .await?;
                println!("Bulk update completed successfully!");
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
            }
        },
        Commands::Authors { cmd } => match cmd {
            AuthorCommands::List => {
                let authors_resp = client.get_authors().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&authors_resp)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Name", "Books"]);

                    let authors = if let Some(a) = authors_resp.get("authors") {
                        a.as_array().cloned().unwrap_or_default()
                    } else if authors_resp.is_array() {
                        authors_resp.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![authors_resp]
                    };

                    for author in authors {
                        table.add_row(vec![
                            author.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            author.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            &author
                                .get("numBooks")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0)
                                .to_string(),
                        ]);
                    }
                    println!("{}", table);
                }
            }
            AuthorCommands::Get { id } => {
                let author = client.get_author(&id).await?;
                println!("{}", serde_json::to_string_pretty(&author)?);
            }
        },
        Commands::Collections { cmd } => match cmd {
            CollectionCommands::List => {
                let collections_resp = client.get_collections().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&collections_resp)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Name", "Items"]);

                    let collections = if let Some(c) = collections_resp.get("collections") {
                        c.as_array().cloned().unwrap_or_default()
                    } else if collections_resp.is_array() {
                        collections_resp.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![collections_resp]
                    };

                    for col in collections {
                        table.add_row(vec![
                            col.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            col.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            &col.get("numItems")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0)
                                .to_string(),
                        ]);
                    }
                    println!("{}", table);
                }
            }
            CollectionCommands::Get { id } => {
                let collection = client.get_collection(&id).await?;
                println!("{}", serde_json::to_string_pretty(&collection)?);
            }
        },
        Commands::Playlists { cmd } => match cmd {
            PlaylistCommands::List => {
                let playlists_resp = client.get_playlists().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&playlists_resp)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Name", "Items"]);

                    let playlists = if let Some(p) = playlists_resp.get("playlists") {
                        p.as_array().cloned().unwrap_or_default()
                    } else if playlists_resp.is_array() {
                        playlists_resp.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![playlists_resp]
                    };

                    for pl in playlists {
                        table.add_row(vec![
                            pl.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            pl.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            &pl.get("numItems")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0)
                                .to_string(),
                        ]);
                    }
                    println!("{}", table);
                }
            }
            PlaylistCommands::Get { id } => {
                let playlist = client.get_playlist(&id).await?;
                println!("{}", serde_json::to_string_pretty(&playlist)?);
            }
        },
        Commands::Series { cmd } => match cmd {
            SeriesCommands::List => {
                let series_resp = client.get_series_list().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&series_resp)?);
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["ID", "Name", "Items"]);

                    let series_list = if let Some(s) = series_resp.get("series") {
                        s.as_array().cloned().unwrap_or_default()
                    } else if series_resp.is_array() {
                        series_resp.as_array().cloned().unwrap_or_default()
                    } else {
                        vec![series_resp]
                    };

                    for s in series_list {
                        table.add_row(vec![
                            s.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            s.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"),
                            &s.get("numBooks")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0)
                                .to_string(),
                        ]);
                    }
                    println!("{}", table);
                }
            }
            SeriesCommands::Get { id } => {
                let series = client.get_series(&id).await?;
                println!("{}", serde_json::to_string_pretty(&series)?);
            }
        },
        Commands::Metadata { cmd } => match cmd {
            MetadataCommands::Tags => {
                let tags_resp = client.get_tags().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&tags_resp)?);
                } else {
                    let tags = tags_resp
                        .get("tags")
                        .and_then(|t| t.as_array())
                        .cloned()
                        .unwrap_or_default();
                    if tags.is_empty() {
                        println!("No tags found.");
                    } else {
                        let mut table = Table::new();
                        table.set_header(vec!["Tag"]);
                        for tag in tags {
                            table.add_row(vec![tag.as_str().unwrap_or("N/A")]);
                        }
                        println!("{}", table);
                    }
                }
            }
            MetadataCommands::Genres => {
                let genres_resp = client.get_genres().await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&genres_resp)?);
                } else {
                    let genres = genres_resp
                        .get("genres")
                        .and_then(|g| g.as_array())
                        .cloned()
                        .unwrap_or_default();
                    if genres.is_empty() {
                        println!("No genres found.");
                    } else {
                        let mut table = Table::new();
                        table.set_header(vec!["Genre"]);
                        for genre in genres {
                            table.add_row(vec![genre.as_str().unwrap_or("N/A")]);
                        }
                        println!("{}", table);
                    }
                }
            }
        },
        Commands::Me => {
            let me = client.get_me().await?;
            println!("{}", serde_json::to_string_pretty(&me)?);
        }
        Commands::Info => {
            let status = client.get_status().await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&status)?);
            } else {
                let mut table = Table::new();
                table.set_header(vec!["Property", "Value"]);

                table.add_row(vec![
                    "Initialized",
                    &status
                        .get("isInit")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                        .to_string(),
                ]);
                table.add_row(vec![
                    "Default Language",
                    status
                        .get("defaultLanguage")
                        .and_then(|v| v.as_str())
                        .unwrap_or("N/A"),
                ]);
                table.add_row(vec![
                    "Config Path",
                    status
                        .get("configPath")
                        .and_then(|v| v.as_str())
                        .unwrap_or("N/A"),
                ]);
                table.add_row(vec![
                    "Metadata Path",
                    status
                        .get("metadataPath")
                        .and_then(|v| v.as_str())
                        .unwrap_or("N/A"),
                ]);

                println!("{table}");
            }
        }
        Commands::Search { query } => {
            let results = client.search(&query).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                let mut table = Table::new();
                table.set_header(vec!["Type", "ID", "Name/Title"]);

                // Access individual result categories
                let categories = [
                    "book",
                    "podcast",
                    "author",
                    "series",
                    "collection",
                    "playlist",
                ];
                for cat in categories {
                    if let Some(items) = results.get(cat).and_then(|v| v.as_array()) {
                        for item in items {
                            let name = item
                                .get("name")
                                .and_then(|v| v.as_str())
                                .or_else(|| {
                                    item.get("media")
                                        .and_then(|m| m.get("metadata"))
                                        .and_then(|meta| meta.get("title"))
                                        .and_then(|v| v.as_str())
                                })
                                .unwrap_or("N/A");

                            table.add_row(vec![
                                cat,
                                item.get("id").and_then(|v| v.as_str()).unwrap_or("N/A"),
                                name,
                            ]);
                        }
                    }
                }

                if table.is_empty() {
                    println!("No results found for '{}'", query);
                } else {
                    println!("{table}");
                }
            }
        }
        Commands::Upload {
            files,
            library,
            folder,
            title,
            author,
            series,
        } => {
            let mut folder_id = folder;
            if folder_id.is_none() {
                // Try to auto-resolve
                let libs_resp = client.get_libraries().await?;
                let libraries = if let Some(l) = libs_resp.get("libraries") {
                    l.as_array().cloned().unwrap_or_default()
                } else if libs_resp.is_array() {
                    libs_resp.as_array().cloned().unwrap_or_default()
                } else {
                    vec![libs_resp]
                };

                let target_lib = libraries.iter().find(|l| l["id"] == library);
                if let Some(lib) = target_lib {
                    if let Some(folders) = lib["folders"].as_array() {
                        if folders.len() == 1 {
                            folder_id = folders[0]["id"].as_str().map(|s| s.to_string());
                            if let Some(ref fid) = folder_id {
                                println!(
                                    "Auto-resolved to folder: {} ({})",
                                    folders[0]["fullPath"].as_str().unwrap_or("N/A"),
                                    fid
                                );
                            }
                        } else if folders.len() > 1 {
                            anyhow::bail!(
                                "Library has multiple folders. Please specify --folder ID."
                            );
                        } else {
                            anyhow::bail!("Library has no folders.");
                        }
                    }
                } else {
                    anyhow::bail!("Library not found.");
                }
            }

            let fid = folder_id.context(
                "Folder ID is required. Use --folder or ensure library has only one folder.",
            )?;

            println!(
                "Uploading {} file(s) to library {} (folder {})...",
                files.len(),
                library,
                fid
            );
            let result = client
                .upload(&library, &fid, &title, author, series, files)
                .await?;
            println!("Upload successful!");
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
        }
        Commands::Auth { cmd } => match cmd {
            AuthCommands::Login { api_key } => {
                let entry = keyring::Entry::new("audiobookshelf-cli", "api_key")
                    .context("Failed to access system keyring")?;
                entry
                    .set_password(&api_key)
                    .context("Failed to save API key to keyring")?;
                println!("API key saved securely to system keyring.");
            }
            AuthCommands::Logout => {
                let entry = keyring::Entry::new("audiobookshelf-cli", "api_key")
                    .context("Failed to access system keyring")?;
                entry
                    .delete_credential()
                    .context("Failed to remove API key from keyring (perhaps it wasn't saved?)")?;
                println!("API key removed from system keyring.");
            }
        },
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        }
    }

    Ok(())
}
