// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use error::{Error, Result};

use std::vec;

mod consts;
mod error;
mod ipc;
mod model;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ipc::select_profile,
            ipc::request_games_update,
            ipc::request_profiles_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
