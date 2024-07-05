use std::{
    fs,
    sync::{Mutex, OnceLock},
};

use rusqlite::Connection;
use shared::db::init_db;
use tauri::{AppHandle, Manager, State};

pub struct ApplicationState {
    pub db: OnceLock<Mutex<Connection>>,
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
