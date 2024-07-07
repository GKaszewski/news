use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::path::Path;

use crate::news::NewsItem;
use crate::rss_feeds::get_all_rss_items;
use crate::rss_feeds::RssItem;

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Pool(#[from] r2d2::Error),
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn init(path: &Path) -> Result<DbPool> {
    let manager = SqliteConnectionManager::file(path);
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get()?;

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

    Ok(pool)
}

pub fn store_rss_items(db: &DbPool, items: &[RssItem]) -> anyhow::Result<()> {
    let existing_items = get_all_rss_items(db)?;
    for item in items {
        if existing_items.iter().any(|i| i.link == item.link) {
            continue;
        }
        db.get()?.execute(
            "INSERT INTO rss_items (title, link, description, pub_date, source) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![item.title, item.link, item.description, item.pub_date, item.source],
        )?;
    }

    Ok(())
}

pub fn store_news_item(db: DbPool, item: NewsItem) -> Result<()> {
    db.get()?.execute(
        "INSERT INTO news_items (title, author, body, url) VALUES (?1, ?2, ?3, ?4)",
        params![item.title, item.author, item.body, item.url],
    )?;

    Ok(())
}
