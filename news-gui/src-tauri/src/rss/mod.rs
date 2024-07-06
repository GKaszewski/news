use anyhow::Result;
use rusqlite::Connection;
use shared::{
    db::store_rss_items,
    rss_feeds::{add_feed_url, fetch_rss_from_feeds, get_all_rss_items, RssItem},
};

pub fn populate_rss_feeds(db: &Connection) {
    let urls: Vec<(String, String)> = vec![
        ("https://feeds.bbci.co.uk/news/world/rss.xml".to_string(), "BBC World News".to_string()),
        ("https://www.nytimes.com/svc/collections/v1/publish/https://www.nytimes.com/section/world/rss.xml".to_string(), "New York Times World News".to_string()),
        ("https://rss.gazeta.pl/pub/rss/najnowsze_wyborcza.xml".to_string(), "Gazeta Wyborcza".to_string()),
    ];

    for (url, name) in urls {
        add_feed_url(&db, &url, &name).expect("Failed to add feed URL");
    }
}

pub async fn populate_rss_items(db: &Connection) -> Result<()> {
    let rss_map = fetch_rss_from_feeds(&db).await?;

    for (url, items) in rss_map {
        store_rss_items(&db, &items).expect("Failed to store RSS items");
        println!("Fetched {} items from {}", items.len(), url);
    }

    Ok(())
}

pub async fn get_rss_items(db: &Connection) -> Result<Vec<RssItem>> {
    let rss_items = get_all_rss_items(&db);
    if let Ok(items) = rss_items {
        if items.len() == 0 {
            let _ = populate_rss_items(&db).await;
            return get_all_rss_items(&db);
        } else {
            println!("Found {} items in the database", items.len());
            return Ok(items);
        }
    } else {
        let _ = populate_rss_items(&db).await;
        return get_all_rss_items(&db);
    }
}
