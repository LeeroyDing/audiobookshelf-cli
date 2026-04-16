use reqwest::{Client, RequestBuilder, Method};
use anyhow::{Result, Context};

pub struct AbsClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl AbsClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            // Ensure base_url does not have a trailing slash for consistent route appending
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            client: Client::new(),
        }
    }

    /// Helper to construct a request with authorization header appropriately set
    pub fn request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        self.client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", self.api_key))
    }

    // Ping endpoint doesn't strictly need auth, but using the normal builder is fine
    pub async fn ping(&self) -> Result<()> {
        let resp = self.request(Method::GET, "/ping").send().await?;
        
        if resp.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!("Ping failed with status: {}", resp.status())
        }
    }

    pub async fn get_libraries(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/libraries").send().await?;
        
        if resp.status().is_success() {
            let json = resp.json::<serde_json::Value>().await.context("Failed to parse JSON")?;
            Ok(json)
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get libraries: {} - {}", status, text)
        }
    }

    pub async fn get_users(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/users").send().await?;
        
        if resp.status().is_success() {
            let json = resp.json::<serde_json::Value>().await.context("Failed to parse JSON")?;
            Ok(json)
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get users: {} - {}", status, text)
        }
    }

    pub async fn get_library_items(&self, library_id: &str) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, &format!("/api/libraries/{}/items", library_id)).send().await?;
        
        if resp.status().is_success() {
            let json = resp.json::<serde_json::Value>().await.context("Failed to parse JSON")?;
            Ok(json)
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get library items: {} - {}", status, text)
        }
    }

    pub async fn get_item(&self, item_id: &str) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, &format!("/api/items/{}", item_id)).send().await?;
        
        if resp.status().is_success() {
            let json = resp.json::<serde_json::Value>().await.context("Failed to parse JSON")?;
            Ok(json)
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get item: {} - {}", status, text)
        }
    }

    pub async fn get_me(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/me").send().await?;
        
        if resp.status().is_success() {
            let json = resp.json::<serde_json::Value>().await.context("Failed to parse JSON")?;
            Ok(json)
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get user info (me): {} - {}", status, text)
        }
    }

    pub async fn get_authors(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/authors").send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get authors: {}", resp.status())
        }
    }

    pub async fn get_author(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, &format!("/api/authors/{}", id)).send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get author {}: {}", id, resp.status())
        }
    }

    pub async fn get_collections(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/collections").send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get collections: {}", resp.status())
        }
    }

    pub async fn get_collection(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, &format!("/api/collections/{}", id)).send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get collection {}: {}", id, resp.status())
        }
    }

    pub async fn get_playlists(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/playlists").send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get playlists: {}", resp.status())
        }
    }

    pub async fn get_playlist(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, &format!("/api/playlists/{}", id)).send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get playlist {}: {}", id, resp.status())
        }
    }

    pub async fn get_series_list(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/series").send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get series: {}", resp.status())
        }
    }

    pub async fn get_series(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, &format!("/api/series/{}", id)).send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get series {}: {}", id, resp.status())
        }
    }

    pub async fn get_tags(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/tags").send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get tags: {}", resp.status())
        }
    }

    pub async fn get_genres(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/genres").send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            anyhow::bail!("Failed to get genres: {}", resp.status())
        }
    }
}
