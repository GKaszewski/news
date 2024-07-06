use anyhow::Result;
use shared::{
    db::{store_rss_items, DbPool},
    rss_feeds::{add_feed_url, fetch_rss_from_feeds, get_all_rss_items, RssItem},
};

pub fn populate_rss_feeds(db: &DbPool) {
    let urls: Vec<(String, String)> = vec![
        ("https://feeds.bbci.co.uk/news/world/rss.xml".to_string(), "BBC World News".to_string()),
        ("https://www.nytimes.com/svc/collections/v1/publish/https://www.nytimes.com/section/world/rss.xml".to_string(), "New York Times World News".to_string()),
        ("https://rss.gazeta.pl/pub/rss/najnowsze_wyborcza.xml".to_string(), "Gazeta Wyborcza".to_string()),
    ];

    for (url, name) in urls {
        add_feed_url(db, &url, &name).expect("Failed to add feed URL");
    }
}

pub async fn populate_rss_items(db: DbPool) -> Result<()> {
    let rss_map = fetch_rss_from_feeds(&db).await?;

    for (_, items) in rss_map {
        store_rss_items(&db, &items).expect("Failed to store RSS items");
    }

    Ok(())
}

pub async fn get_rss_items(db: DbPool) -> Result<Vec<RssItem>> {
    let rss_items = get_all_rss_items(&db);
    let Ok(items) = rss_items else {
        let _ = populate_rss_items(db.clone()).await;
        return get_all_rss_items(&db);
    };

    if items.is_empty() {
        let _ = populate_rss_items(db.clone()).await;
        get_all_rss_items(&db)
    } else {
        Ok(items)
    }
}
