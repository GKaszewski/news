use std::sync::Mutex;
use tauri::State;

use shared::rss_feeds::{get_all_feed_urls, get_all_rss_items, FeedUrl, RssItem};

pub struct RssFeedsState {
    pub feeds: Mutex<Vec<FeedUrl>>,
    pub items: Mutex<Vec<RssItem>>,
}
