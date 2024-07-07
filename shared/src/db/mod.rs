use std::path::Path;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::news::NewsItem;
use crate::rss_feeds::get_all_rss_items;
use crate::rss_feeds::RssItem;

pub async fn init(path: &Path) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(path.to_str().unwrap()).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn store_rss_items(db: &SqlitePool, items: &[RssItem]) -> Result<()> {
    let existing_items = get_all_rss_items(db).await?;
    for item in items {
        if existing_items.iter().any(|i| i.link == item.link) {
            continue;
        }
        let mut conn = db.acquire().await?;
        sqlx::query!(
            "INSERT INTO rss_items (title, link, description, pub_date, source) VALUES (?1, ?2, ?3, ?4, ?5)",
            item.title,
            item.link,
            item.description,
            item.pub_date,
            item.source
        ).execute(&mut *conn).await?;
    }

    Ok(())
}

pub async fn store_news_item(db: &SqlitePool, item: NewsItem) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!(
        "INSERT INTO news_items (title, author, body, url) VALUES (?1, ?2, ?3, ?4)",
        item.title,
        item.author,
        item.body,
        item.url
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
