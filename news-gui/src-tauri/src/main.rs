// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::{initialize_database, ApplicationState};
use std::sync::{Mutex, OnceLock};

use rss::{get_news_item_from_url, get_rss_items, populate_rss_feeds};
use shared::{
    news::NewsItem,
    rss_feeds::{get_all_feed_urls, FeedUrl, RssItem},
};
use tauri::{AppHandle, Manager, State};

pub mod core;
pub mod rss;

#[tauri::command]
fn get_feeds(app_handle: AppHandle) -> Vec<FeedUrl> {
    let app_state: State<ApplicationState> = app_handle.state();
    let feeds = app_state.feeds.lock().unwrap();
    feeds.clone()
}

#[tauri::command]
async fn get_rss_items_command(app_handle: AppHandle) -> Vec<RssItem> {
    let app_state: State<ApplicationState> = app_handle.state();
    let db = app_state.db();

    let Ok(items) = get_rss_items(db.clone()).await else {
        return [].to_vec();
    };
    app_state.items.lock().unwrap().clone_from(&items);
    items
}

#[tauri::command]
async fn get_news_from_url_command(url: String, app_handle: AppHandle) -> NewsItem {
    let app_state: State<ApplicationState> = app_handle.state();
    let db = app_state.db();
    let Ok(item) = get_news_item_from_url(db.clone(), url).await else {
        return NewsItem::default();
    };

    item
}

fn main() {
    tauri::Builder::default()
        .manage(ApplicationState {
            db: OnceLock::new(),
            feeds: Mutex::new(Vec::new()),
            items: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_feeds,
            get_rss_items_command,
            get_news_from_url_command
        ])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<ApplicationState> = handle.state();
            let db = initialize_database(&handle).expect("Failed to initialize database");
            app_state.db.set(db).expect("Failed to set database");

            let db = app_state.db();
            populate_rss_feeds(db);
            let feed_urls = get_all_feed_urls(db).expect("Failed to get all feed URLs");
            *app_state.feeds.lock().unwrap() = feed_urls;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
