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
            ipc::play_game,
            ipc::get_full_profile,
            ipc::select_profile,
            ipc::select_profile_version,
            ipc::list_game_profiles,
            ipc::list_versions,
            ipc::list_games,
            ipc::select_dir,
            ipc::get_settings,
            ipc::set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
