use std::{fs, sync::Mutex};

use rusqlite::Connection;
use shared::db::init_db;
use tauri::{AppHandle, Manager, State};

pub struct ApplicationState {
    pub db: Mutex<Option<Connection>>,
}

pub trait ServiceAccess {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult,
    {
        let app_state: State<ApplicationState> = self.state();
        let db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_ref().unwrap();

        operation(db)
    }

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult,
    {
        let app_state: State<ApplicationState> = self.state();
        let mut db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
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
