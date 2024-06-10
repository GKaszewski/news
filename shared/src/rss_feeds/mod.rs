use std::collections::HashMap;

use anyhow::Result;
use rss::Channel;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedUrl {
    pub id: i32,
    pub url: String,
    pub name: String,
}

pub async fn fetch_rss_feed(url: &str, feed_name: &str) -> Result<Vec<RssItem>> {
    let content = reqwest::get(url).await?.bytes().await?;
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

pub fn delete_rss_feed(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM feed_urls WHERE id = ?1", [id])?;
    Ok(())
}

pub fn clear_rss_items(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM rss_items", [])?;
    Ok(())
}

pub fn add_feed_url(conn: &Connection, url: &str, name: &str) -> Result<()> {
    let existing_urls = get_all_feed_urls(conn)?;
    if existing_urls.iter().any(|u| u.url == url) {
        return Ok(());
    }
    conn.execute(
        "INSERT INTO feed_urls (url, name) VALUES (?1, ?2)",
        [url, name],
    )?;
    Ok(())
}

pub fn update_feed_url(conn: &Connection, id: i32, url: &str, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE feed_urls SET url = ?1, name = ?2 WHERE id = ?3",
        params![url, name, id],
    )?;
    Ok(())
}

pub fn delete_feed_url(conn: &Connection, url: &str) -> Result<()> {
    conn.execute("DELETE FROM feed_urls WHERE url = ?1", [url])?;
    Ok(())
}

pub fn clear_feed_urls(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM feed_urls", [])?;
    Ok(())
}

pub fn get_all_feed_urls(conn: &Connection) -> Result<Vec<FeedUrl>> {
    let mut stmt = conn.prepare("SELECT id, url, name FROM feed_urls")?;
    let rows = stmt.query_map([], |row| {
        Ok(FeedUrl {
            id: row.get(0)?,
            url: row.get(1)?,
            name: row.get(2)?,
        })
    })?;

    let mut feeds = Vec::new();
    for feed in rows {
        feeds.push(feed?);
    }

    Ok(feeds)
}

pub async fn fetch_rss_from_feeds(conn: &Connection) -> Result<HashMap<String, Vec<RssItem>>> {
    let feed_urls = get_all_feed_urls(conn)?;
    let mut rss_map = HashMap::new();
    for feed_url in feed_urls {
        let items = fetch_rss_feed(&feed_url.url, &feed_url.name).await?;
        rss_map.insert(feed_url.name, items);
    }

    Ok(rss_map)
}

pub fn get_all_rss_items(conn: &Connection) -> Result<Vec<RssItem>> {
    let mut stmt =
        conn.prepare("SELECT title, link, description, pub_date, source FROM rss_items")?;
    let rows = stmt.query_map([], |row| {
        Ok(RssItem {
            title: row.get(0)?,
            link: row.get(1)?,
            description: row.get(2)?,
            pub_date: row.get(3)?,
            source: row.get(4)?,
        })
    })?;

    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }

    Ok(items)
}

pub fn filter_rss_items_by_title(conn: &Connection, title: &str) -> Result<Vec<RssItem>> {
    let mut stmt = conn.prepare(
        "SELECT title, link, description, pub_date, source FROM rss_items WHERE title LIKE ?1",
    )?;
    let rows = stmt.query_map([format!("%{}%", title)], |row| {
        Ok(RssItem {
            title: row.get(0)?,
            link: row.get(1)?,
            description: row.get(2)?,
            pub_date: row.get(3)?,
            source: row.get(4)?,
        })
    })?;

    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }

    Ok(items)
}

pub fn filter_rss_items_by_source(conn: &Connection, source: &str) -> Result<Vec<RssItem>> {
    let mut stmt = conn.prepare(
        "SELECT title, link, description, pub_date, source FROM rss_items WHERE source LIKE ?1",
    )?;
    let rows = stmt.query_map([format!("%{}%", source)], |row| {
        Ok(RssItem {
            title: row.get(0)?,
            link: row.get(1)?,
            description: row.get(2)?,
            pub_date: row.get(3)?,
            source: row.get(4)?,
        })
    })?;

    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }

    Ok(items)
}
