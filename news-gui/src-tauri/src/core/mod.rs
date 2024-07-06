use std::{
    fs,
    sync::{Mutex, MutexGuard, OnceLock},
};

use rusqlite::Connection;
use shared::{
    db::init_db,
    rss_feeds::{FeedUrl, RssItem},
};
use tauri::AppHandle;

pub struct ApplicationState {
    pub db: OnceLock<Mutex<Connection>>,
    pub feeds: Mutex<Vec<FeedUrl>>,
    pub items: Mutex<Vec<RssItem>>,
}

impl ApplicationState {
    pub fn db(&self) -> MutexGuard<'_, Connection> {
        self.db.get().unwrap().lock().unwrap()
    }
}

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data directory");
    fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
    let sqlite_path = app_dir.join("news.db");
    let db = init_db(&sqlite_path)?;

    Ok(db)
}
