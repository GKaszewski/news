// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::{initialize_database, ApplicationState};
use std::sync::{Mutex, OnceLock};

use tauri::{Manager, State};

pub mod core;
pub mod rss;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(rss::RssFeedsState {
            feeds: Mutex::new(Vec::new()),
            items: Mutex::new(Vec::new()),
        })
        .manage(ApplicationState {
            db: OnceLock::new(),
        })
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<ApplicationState> = handle.state();
            let db = initialize_database(&handle).expect("Failed to initialize database");
            app_state
                .db
                .set(Mutex::new(db))
                .expect("Failed to set database");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
