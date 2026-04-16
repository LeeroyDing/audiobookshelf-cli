use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesResponse {
    pub libraries: Vec<Library>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "type")]
    pub user_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersResponse {
    // The users response is a direct array if looking at some endpoints,
    // but often Audiobookshelf puts things in JSON objects.
    // Let's defer to the commands module logic (e.g., parsing raw json to find out).
}
