use anyhow::Result;
use rusqlite::params;
use rusqlite::Connection;

use crate::rss_feeds::get_all_rss_items;
use crate::rss_feeds::RssItem;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("rss_feeds.db")?;
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
