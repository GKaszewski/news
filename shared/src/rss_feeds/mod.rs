use std::collections::HashMap;

use anyhow::{bail, Result};
use rss::Channel;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct FeedUrl {
    pub id: i32,
    pub url: String,
    pub name: String,
}

pub async fn fetch_rss_feed(url: &str, feed_name: &str) -> Result<Vec<RssItem>> {
    let content = reqwest::get(url).await?.bytes().await?;
    if content.is_empty() {
        bail!(format!("Failed to fetch rss feed {:?}", feed_name));
    }
    let channel = Channel::read_from(&content[..])?;

    let items = channel
        .items()
        .iter()
        .map(|item| RssItem {
            title: item.title().unwrap_or("").to_string(),
            link: item.link().unwrap_or("").to_string(),
            description: item.description().map(|s| s.to_string()),
            pub_date: item.pub_date().map(|s| s.to_string()),
            source: Some(feed_name.to_string()),
        })
        .collect();

    Ok(items)
}

pub async fn delete_rss_feed(db: &SqlitePool, id: i32) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!("DELETE FROM rss_items WHERE source = ?1", id)
        .execute(&mut *conn)
        .await?;

    Ok(())
}

pub async fn clear_rss_items(db: &SqlitePool) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!("DELETE FROM rss_items")
        .execute(&mut *conn)
        .await?;

    Ok(())
}

pub async fn add_feed_url(db: &SqlitePool, url: &str, name: &str) -> Result<()> {
    let existing_urls = get_all_feed_urls(db).await?;
    if existing_urls.iter().any(|u| u.url == url) {
        return Ok(());
    }

    let mut conn = db.acquire().await?;
    sqlx::query!(
        "INSERT INTO feed_urls (url, name) VALUES (?1, ?2)",
        url,
        name
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn update_feed_url(db: &SqlitePool, id: i32, url: &str, name: &str) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!(
        "UPDATE feed_urls SET url = ?1, name = ?2 WHERE id = ?3",
        url,
        name,
        id
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn delete_feed_url(db: &SqlitePool, url: &str) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!("DELETE FROM feed_urls WHERE url = ?1", url)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

pub async fn clear_feed_urls(db: &SqlitePool) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!("DELETE FROM feed_urls")
        .execute(&mut *conn)
        .await?;
    Ok(())
}

pub async fn get_all_feed_urls(db: &SqlitePool) -> Result<Vec<FeedUrl>> {
    let query = sqlx::query_as::<_, FeedUrl>("SELECT id, url, name FROM feed_urls");
    let feed_urls: Vec<FeedUrl> = query.fetch_all(db).await?;

    Ok(feed_urls)
}

pub async fn fetch_rss_from_feeds(db: &SqlitePool) -> Result<HashMap<String, Vec<RssItem>>> {
    let feed_urls = get_all_feed_urls(db).await?;
    let mut rss_map = HashMap::new();
    for feed_url in feed_urls {
        let items = fetch_rss_feed(&feed_url.url, &feed_url.name).await?;
        rss_map.insert(feed_url.name, items);
    }

    Ok(rss_map)
}

pub async fn get_all_rss_items(db: &SqlitePool) -> Result<Vec<RssItem>> {
    let query = sqlx::query_as::<_, RssItem>(
        "SELECT title, link, description, pub_date, source FROM rss_items",
    );
    let rss_items: Vec<RssItem> = query.fetch_all(db).await?;

    Ok(rss_items)
}

pub async fn filter_rss_items_by_title(db: &SqlitePool, title: &str) -> Result<Vec<RssItem>> {
    let query = sqlx::query_as::<_, RssItem>(
        "SELECT title, link, description, pub_date, source FROM rss_items WHERE title LIKE ?1",
    );
    let rss_items: Vec<RssItem> = query.bind(format!("%{}%", title)).fetch_all(db).await?;

    Ok(rss_items)
}

pub async fn filter_rss_items_by_source(db: &SqlitePool, source: &str) -> Result<Vec<RssItem>> {
    let query = sqlx::query_as::<_, RssItem>(
        "SELECT title, link, description, pub_date, source FROM rss_items WHERE source LIKE ?1",
    );
    let rss_items: Vec<RssItem> = query.bind(format!("%{}%", source)).fetch_all(db).await?;

    Ok(rss_items)
}
