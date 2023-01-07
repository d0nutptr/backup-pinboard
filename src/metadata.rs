use anyhow::{Result, Context, bail};
use hyper::body::Bytes;

use reqwest;

const PINBOARD_API_URL: &'static str = "https://api.pinboard.in/v1/posts/all?format=json";

/// Return metadata about all your posts from the Pinboard API.
///
///  - `username`: Pinboard username
///  - `password`: Pinboard password
///
pub async fn get_metadata(username: String, password: String) -> Result<Bytes> {
    let client = reqwest::Client::new();
    let resp = client
        .get(PINBOARD_API_URL)
        .basic_auth(username, Some(password))
        .send()
        .await
        .context("Unexpected error calling the Pinboard API")?;

    if !resp.status().is_success() {
        bail!("Error status code from the Pinboard API: {}", resp.status())
    }

    resp
        .bytes()
        .await
        .context("Failed to read body of Pinboard API response")
}
