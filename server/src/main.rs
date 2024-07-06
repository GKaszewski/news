use std::path::PathBuf;

use shared::{
    db::{init, store_news_item, store_rss_items, DbPool},
    news::fetch_news_from_url,
    rss_feeds::{
        add_feed_url, clear_feed_urls, fetch_rss_from_feeds, filter_rss_items_by_source,
        get_all_rss_items,
    },
};

struct State {
    db: DbPool,
}

async fn setup_rss_feeds(state: &State) {
    let db = &state.db;
    let urls: Vec<(String, String)> = vec![
        ("https://feeds.bbci.co.uk/news/world/rss.xml".to_string(), "BBC World News".to_string()),
        ("https://www.nytimes.com/svc/collections/v1/publish/https://www.nytimes.com/section/world/rss.xml".to_string(), "New York Times World News".to_string()),
        ("https://www.aljazeera.com/xml/rss/all.xml".to_string(), "Al Jazeera".to_string()),
        ("https://rss.gazeta.pl/pub/rss/najnowsze_wyborcza.xml".to_string(), "Gazeta Wyborcza".to_string()),
    ];

    clear_feed_urls(db).expect("Failed to clear feed URLs");

    for (url, name) in urls {
        add_feed_url(db, &url, &name).expect("Failed to add feed URL");
    }

    let rss_map = fetch_rss_from_feeds(db)
        .await
        .expect("Failed to fetch RSS feeds");

    for (url, items) in rss_map {
        store_rss_items(db, &items).expect("Failed to store RSS items");
        println!("Fetched {} items from {}", items.len(), url);
    }

    let items = get_all_rss_items(db).expect("Failed to get all RSS items");

    for item in items.iter() {
        println!("{:?}, {}, {}", item.source, item.title, item.link);
    }

    println!("Done!");
}

// this is test code for now

#[tokio::main]
async fn main() {
    let db_path = PathBuf::from("news.db");
    let state = State {
        db: init(&db_path).expect("Failed to initialize database"),
    };
    setup_rss_feeds(&state).await;

    let bbc_items = filter_rss_items_by_source(&state.db, "BBC World News")
        .expect("Failed to filter RSS items by source");

    if let Some(item) = bbc_items.get(2) {
        println!("{:?}", item);

        let news = fetch_news_from_url(state.db.clone(), &item.link, shared::news::NewsSource::BBC)
            .await
            .expect("Failed to fetch news from URL");

        store_news_item(&state.db, &news).expect("Failed to store news item");
        println!("{:?}", news);
    }
}
