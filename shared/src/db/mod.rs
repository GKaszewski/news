use std::path::PathBuf;

use anyhow::Result;
use rusqlite::params;
use rusqlite::Connection;

use crate::news::NewsItem;
use crate::rss_feeds::get_all_rss_items;
use crate::rss_feeds::RssItem;

pub fn init_db(path: &PathBuf) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS rss_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        link TEXT NOT NULL,
        description TEXT,
        pub_date TEXT,
        source TEXT)
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS feed_urls (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        name TEXT NOT NULL)
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS news_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        author TEXT,
        body TEXT,
        url TEXT NOT NULL)",
        [],
    )?;

    Ok(conn)
}

pub fn store_rss_items(conn: &Connection, items: &[RssItem]) -> Result<()> {
    let existing_items = get_all_rss_items(conn)?;
    for item in items {
        if existing_items.iter().any(|i| i.link == item.link) {
            continue;
        }
        conn.execute(
            "INSERT INTO rss_items (title, link, description, pub_date, source) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![item.title, item.link, item.description, item.pub_date, item.source],
        )?;
    }

    Ok(())
}

pub fn store_news_item(conn: &Connection, item: &NewsItem) -> Result<()> {
    conn.execute(
        "INSERT INTO news_items (title, author, body, url) VALUES (?1, ?2, ?3, ?4)",
        params![item.title, item.author, item.body, item.url],
    )?;

    Ok(())
}
