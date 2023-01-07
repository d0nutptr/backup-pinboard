use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::{Result, bail, Context};
use hyper::body::HttpBody;

use reqwest::header::{HeaderValue, LOCATION, SET_COOKIE};
use reqwest::Client;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::redirect::Policy;
use soup::{NodeExt, QueryBuilderExt, Soup};


/// Log in to Pinboard and get a login cookie that can be used on subsequent
/// requests.
///
async fn get_login_cookie(username: &str, password: &str) -> Result<Jar> {
    // Start by logging in to Pinboard, so we have the appropriate cookies.
    // Because Pinboard sends us into a weird redirect loop, we have to
    // tell reqwest not to follow redirects, just check the redirect worked.
    let client = Client::builder()
        .redirect(Policy::none())
        .build()?;

    let response = client
        .post("https://pinboard.in/auth/")
        .form(&[("username", &username), ("password", &password)])
        .send()
        .await?;

    if !matches!(
        response.headers().get(LOCATION),
        Some(location) if location != HeaderValue::from_static("?error=wrong+password")
    ) {
        bail!("Error logging in to Pinboard!")
    }

    let mut set_cookie_header = response
        .headers()
        .get_all(SET_COOKIE);

    let cookie_jar = Jar::default();
    cookie_jar.set_cookies(&mut set_cookie_header.iter(), response.url());

    Ok(cookie_jar)
}


/// Given a blob of HTML from a Pinboard index, update the map of cache IDs
/// and corresponding links.
fn get_cache_ids_from_html(html_soup: &Soup) -> HashSet<String>{
    html_soup
        .tag("a")
        .class("cached")
        .recursive(true)
        .find_all()
        .filter_map(|element| element.get("href"))
        .collect()
}


async fn get_html_for_page(client: &Client, path: &str) -> Result<String> {
    let url = format!("https://pinboard.in{}", path);

    let response = client
        .get(&url)
        .send()
        .await?;

    response
        .text()
        .await
        .context("Failed to get html for page")
}


fn get_next_page_path(html_soup: &Soup) -> Option<String> {
    html_soup
        .tag("a")
        .attr("id", "top_earlier")
        .recursive(true)
        .find()
        .and_then(|element| element.get("href"))
}

async fn update_cache_ids_for_path(
    client: &Client,
    path: &str
) -> Result<(HashSet<String>, Option<String>)> {
    let content = get_html_for_page(&client, &path).await?;

    let soup = Soup::new(&content);
    let cache_ids = get_cache_ids_from_html(&soup);
    let next_page = get_next_page_path(&soup);

    Ok((cache_ids, next_page))
}

/// Return a map from URLs to Pinboard cache IDs.
///
///  - `username`: Pinboard username
///  - `password`: Pinboard password
///
pub async fn get_cache_ids(username: &str, password: &str) -> Result<HashSet<String>> {
    let cookie_jar = get_login_cookie(&username, &password).await?;
    let client = Client::builder()
        .cookie_provider(Arc::new(cookie_jar))
        .build()?;

    let mut cache_ids = HashSet::new();

    // Now fetch a blob of HTML for the first page.
    let mut next_path = Some(format!("/u:{}/?per_page=160", username));

    while let Some(path) = &next_path {
        let (
            new_cache_ids,
            new_path
        ) = update_cache_ids_for_path(&client, path).await?;

        cache_ids.extend(new_cache_ids);
        next_path = new_path;
    }

    Ok(cache_ids)
}
