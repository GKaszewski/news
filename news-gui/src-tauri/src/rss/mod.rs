use anyhow::Result;
use shared::{
    db::{store_news_item, store_rss_items},
    news::{fetch_news_from_url, NewsItem},
    rss_feeds::{add_feed_url, fetch_rss_from_feeds, get_all_rss_items, RssItem},
};
use sqlx::SqlitePool;

pub fn populate_rss_feeds(db: &SqlitePool) {
    let urls: Vec<(String, String)> = vec![
        ("https://feeds.bbci.co.uk/news/world/rss.xml".to_string(), "BBC World News".to_string()),
        ("https://www.nytimes.com/svc/collections/v1/publish/https://www.nytimes.com/section/world/rss.xml".to_string(), "New York Times World News".to_string()),
        ("https://rss.gazeta.pl/pub/rss/najnowsze_wyborcza.xml".to_string(), "Gazeta Wyborcza".to_string()),
    ];

    for (url, name) in urls {
        add_feed_url(db, &url, &name).expect("Failed to add feed URL");
    }
}

pub async fn populate_rss_items(db: &SqlitePool) -> Result<()> {
    let rss_map = fetch_rss_from_feeds(&db).await?;

    for (_, items) in rss_map {
        store_rss_items(&db, &items).expect("Failed to store RSS items");
    }

    Ok(())
}

pub async fn get_rss_items(db: &SqlitePool) -> Result<Vec<RssItem>> {
    let rss_items = get_all_rss_items(&db);
    let Ok(items) = rss_items else {
        let _ = populate_rss_items(db).await;
        return get_all_rss_items(&db);
    };

    if items.is_empty() {
        let _ = populate_rss_items(db).await;
        get_all_rss_items(&db)
    } else {
        Ok(items)
    }
}

pub async fn get_news_item_from_url(db: &SqlitePool, url: String) -> Result<NewsItem> {
    let news_item = fetch_news_from_url(db, url.as_str(), shared::news::NewsSource::BBC).await?;
    store_news_item(db, news_item.clone())?;
    Ok(news_item)
}
