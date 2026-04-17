use anyhow::Result;
use reqwest::{Client, Method, RequestBuilder};

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

    /// Helper to handle responses and provide descriptive error messages
    async fn handle_response(&self, resp: reqwest::Response, context: &str) -> Result<serde_json::Value> {
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed == "OK" {
                return Ok(serde_json::Value::Null);
            }
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => Ok(json),
                Err(_) => Ok(serde_json::Value::String(text)),
            }
        } else {
            let text = resp.text().await.unwrap_or_else(|_| "No error message provided".to_string());
            match status.as_u16() {
                401 | 403 => anyhow::bail!("Unauthorized: {}. Please check your AUDIOBOOKSHELF_API_KEY.", context),
                404 => anyhow::bail!("Not Found: {}. The requested resource or endpoint does not exist.", context),
                500..=599 => anyhow::bail!("Server Error: {}. The Audiobookshelf server returned a 5xx error: {}", context, text),
                _ => anyhow::bail!("API Error ({}): {}. Message: {}", status, context, text),
            }
        }
    }

    // Ping endpoint doesn't strictly need auth, but using the normal builder is fine
    pub async fn ping(&self) -> Result<()> {
        let resp = self.request(Method::GET, "/ping").send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            self.handle_response(resp, "while pinging server").await.map(|_| ())
        }
    }

    pub async fn get_libraries(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/libraries").send().await?;
        self.handle_response(resp, "getting libraries").await
    }

    pub async fn get_users(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/users").send().await?;
        self.handle_response(resp, "getting users").await
    }

    pub async fn get_library_items(&self, library_id: &str) -> Result<serde_json::Value> {
        let resp = self
            .request(Method::GET, &format!("/api/libraries/{}/items", library_id))
            .send()
            .await?;
        self.handle_response(resp, &format!("getting items for library {}", library_id)).await
    }

    pub async fn get_item(&self, item_id: &str) -> Result<serde_json::Value> {
        let resp = self
            .request(Method::GET, &format!("/api/items/{}", item_id))
            .send()
            .await?;
        self.handle_response(resp, &format!("getting item {}", item_id)).await
    }

    pub async fn get_me(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/me").send().await?;
        self.handle_response(resp, "getting current user info").await
    }

    pub async fn get_authors(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/authors").send().await?;
        self.handle_response(resp, "getting authors").await
    }

    pub async fn get_author(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self
            .request(Method::GET, &format!("/api/authors/{}", id))
            .send()
            .await?;
        self.handle_response(resp, &format!("getting author {}", id)).await
    }

    pub async fn get_collections(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/collections").send().await?;
        self.handle_response(resp, "getting collections").await
    }

    pub async fn get_collection(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self
            .request(Method::GET, &format!("/api/collections/{}", id))
            .send()
            .await?;
        self.handle_response(resp, &format!("getting collection {}", id)).await
    }

    pub async fn get_playlists(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/playlists").send().await?;
        self.handle_response(resp, "getting playlists").await
    }

    pub async fn get_playlist(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self
            .request(Method::GET, &format!("/api/playlists/{}", id))
            .send()
            .await?;
        self.handle_response(resp, &format!("getting playlist {}", id)).await
    }

    pub async fn get_series_list(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/series").send().await?;
        self.handle_response(resp, "getting series").await
    }

    pub async fn get_series(&self, id: &str) -> Result<serde_json::Value> {
        let resp = self
            .request(Method::GET, &format!("/api/series/{}", id))
            .send()
            .await?;
        self.handle_response(resp, &format!("getting series {}", id)).await
    }

    pub async fn get_tags(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/tags").send().await?;
        self.handle_response(resp, "getting tags").await
    }

    pub async fn get_genres(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/api/genres").send().await?;
        self.handle_response(resp, "getting genres").await
    }

    pub async fn get_status(&self) -> Result<serde_json::Value> {
        let resp = self.request(Method::GET, "/status").send().await?;
        self.handle_response(resp, "getting server status").await
    }

    pub async fn scan_library(&self, id: &str, force: bool) -> Result<serde_json::Value> {
        let mut url = format!("/api/libraries/{}/scan", id);
        if force {
            url.push_str("?force=true");
        }
        let resp = self.request(Method::POST, &url).send().await?;
        self.handle_response(resp, &format!("scanning library {}", id)).await
    }

    pub async fn search(&self, query: &str) -> Result<serde_json::Value> {
        let libs_resp = self.get_libraries().await?;
        let libraries = if let Some(l) = libs_resp.get("libraries") {
            l.as_array().cloned().unwrap_or_default()
        } else if libs_resp.is_array() {
            libs_resp.as_array().cloned().unwrap_or_default()
        } else {
            vec![libs_resp]
        };

        let mut aggregated = serde_json::Map::new();

        for lib in libraries {
            if let Some(id) = lib.get("id").and_then(|v| v.as_str()) {
                let url = format!("/api/libraries/{}/search", id);
                let resp = self.request(Method::GET, &url)
                    .query(&[("q", query)])
                    .send()
                    .await?;
                
                if let Ok(results) = self.handle_response(resp, &format!("searching library {}", id)).await {
                    if let Some(obj) = results.as_object() {
                        for (key, val) in obj {
                            if let Some(arr) = val.as_array() {
                                let existing = aggregated.entry(key.clone()).or_insert_with(|| serde_json::Value::Array(Vec::new()));
                                if let Some(existing_arr) = existing.as_array_mut() {
                                    existing_arr.extend(arr.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(serde_json::Value::Object(aggregated))
    }

    pub async fn update_item_metadata(&self, id: &str, metadata: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("/api/items/{}/media", id);
        let resp = self.request(Method::PATCH, &url)
            .json(&serde_json::json!({ "metadata": metadata }))
            .send()
            .await?;
        self.handle_response(resp, &format!("updating metadata for item {}", id)).await
    }

    pub async fn match_item(&self, id: &str) -> Result<serde_json::Value> {
        let url = format!("/api/items/{}/match", id);
        let resp = self.request(Method::POST, &url).send().await?;
        self.handle_response(resp, &format!("matching item {}", id)).await
    }

    pub async fn unmatch_item(&self, id: &str) -> Result<serde_json::Value> {
        let url = format!("/api/items/{}/match", id);
        let resp = self.request(Method::DELETE, &url).send().await?;
        self.handle_response(resp, &format!("unmatching item {}", id)).await
    }

    pub async fn batch_update_items(&self, ids: &[String], metadata: serde_json::Value) -> Result<serde_json::Value> {
        let payload: Vec<serde_json::Value> = ids.iter().map(|id| {
            serde_json::json!({
                "id": id,
                "mediaPayload": {
                    "metadata": metadata
                }
            })
        }).collect();

        let resp = self.request(Method::POST, "/api/items/batch/update")
            .json(&payload)
            .send()
            .await?;
        self.handle_response(resp, "performing batch update").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_ping_success() {
        let mut server = Server::new_async().await;
        let url = server.url();
        let mock = server.mock("GET", "/ping").with_status(200).create();

        let client = AbsClient::new(url, "fake_key".to_string());
        let result = client.ping().await;

        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_get_libraries_success() {
        let mut server = Server::new_async().await;
        let url = server.url();
        let mock = server
            .mock("GET", "/api/libraries")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"libraries": [{"id": "lib1", "name": "Books"}]}"#)
            .create();

        let client = AbsClient::new(url, "fake_key".to_string());
        let result = client.get_libraries().await.unwrap();

        assert_eq!(result["libraries"][0]["id"], "lib1");
        mock.assert();
    }

    #[tokio::test]
    async fn test_get_me_success() {
        let mut server = Server::new_async().await;
        let url = server.url();
        let mock = server
            .mock("GET", "/api/me")
            .match_header("Authorization", "Bearer fake_key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"username": "leeroy"}"#)
            .create();

        let client = AbsClient::new(url, "fake_key".to_string());
        let result = client.get_me().await.unwrap();

        assert_eq!(result["username"], "leeroy");
        mock.assert();
    }
}
