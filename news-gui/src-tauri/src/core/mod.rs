use std::{
    fs,
    sync::{Mutex, OnceLock},
};

use shared::{
    db::{self},
    rss_feeds::{FeedUrl, RssItem},
};
use sqlx::SqlitePool;
use tauri::AppHandle;

pub struct ApplicationState {
    pub db: OnceLock<SqlitePool>,
    pub feeds: Mutex<Vec<FeedUrl>>,
    pub items: Mutex<Vec<RssItem>>,
}

impl ApplicationState {
    pub fn db(&self) -> &DbPool {
        self.db.get().unwrap()
    }
}

pub fn initialize_database(app_handle: &AppHandle) -> db::Result<DbPool> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data directory");
    fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
    let sqlite_path = app_dir.join("news.db");
    let db = db::init(&sqlite_path)?;

    Ok(db)
}
