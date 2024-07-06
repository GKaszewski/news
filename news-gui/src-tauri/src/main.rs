// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::{initialize_database, ApplicationState};
use std::sync::{Mutex, OnceLock};

use shared::rss_feeds::{add_feed_url, get_all_feed_urls, FeedUrl};
use tauri::{AppHandle, Manager, State};

pub mod core;
pub mod rss;

#[tauri::command]
fn get_feeds(app_handle: AppHandle) -> Vec<FeedUrl> {
    let app_state: State<ApplicationState> = app_handle.state();
    let feeds = app_state.feeds.lock().unwrap();
    feeds.clone()
}

fn main() {
    tauri::Builder::default()
        .manage(ApplicationState {
            db: OnceLock::new(),
            feeds: Mutex::new(Vec::new()),
            items: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![get_feeds])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<ApplicationState> = handle.state();
            let db = initialize_database(&handle).expect("Failed to initialize database");
            let urls: Vec<(String, String)> = vec![
        ("https://feeds.bbci.co.uk/news/world/rss.xml".to_string(), "BBC World News".to_string()),
        ("https://www.nytimes.com/svc/collections/v1/publish/https://www.nytimes.com/section/world/rss.xml".to_string(), "New York Times World News".to_string()),
        ("https://rss.gazeta.pl/pub/rss/najnowsze_wyborcza.xml".to_string(), "Gazeta Wyborcza".to_string()),
        ("https://www.theguardian.com/world/poland/rss".to_string(), "The Guardian".to_string()),
    ];
            app_state
                .db
                .set(Mutex::new(db))
                .expect("Failed to set database");

            let db = app_state.db();
                 for (url, name) in urls {
        add_feed_url(&db, &url, &name).expect("Failed to add feed URL");

        let feed_urls = get_all_feed_urls(&db).expect("Failed to get all feed URLs");
        *app_state.feeds.lock().unwrap() = feed_urls;
    }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
