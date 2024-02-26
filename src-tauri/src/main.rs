// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use error::{Error, Result};

use std::vec;
use log::info;
use tauri::api::cli::Matches;
use tauri_plugin_log::LogTarget;
use crate::utils::curr_dir_path;

mod consts;
mod error;
mod ipc;
mod model;
mod utils;

// TODO: Implement CLI
fn cli_command(name: String, matches: Matches) {
  match name.as_str() {
    "load_version" => {
      info!("{:?}", matches);
      // let version = matches.args.get("version").unwrap();
      // version.value
    }
    _ => {}
  }
}

fn main() {
  let now = chrono::offset::Utc::now();
  let log_name = now.format("%Y-%m-%d_%H-%M-%S").to_string();

  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default().targets([
      LogTarget::Folder(curr_dir_path().join("logs")),
      LogTarget::Stdout,
      LogTarget::Webview,
    ]).log_name(log_name).build())
    .invoke_handler(tauri::generate_handler![
      ipc::play_game,
      ipc::get_full_profile,
      ipc::update_profile,
      ipc::select_profile,
      ipc::list_game_profiles,
      ipc::list_versions,
      ipc::list_games,
      ipc::get_full_game,
      ipc::update_game,
      ipc::select_dir,
      ipc::import_version,
      ipc::get_settings,
      ipc::set_settings,
    ])
    .setup(|app| {
      match app.get_cli_matches() {
        Ok(matches) => {
          if let Some(cmd) = matches.subcommand {
            cli_command(cmd.name, cmd.matches);
          }
        }
        Err(_) => {}
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
