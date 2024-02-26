// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use error::{Error, Result};

use std::vec;
use tauri::api::cli::Matches;

mod consts;
mod error;
mod ipc;
mod model;
mod utils;

// TODO: Implement CLI
fn cli_command(name: String, matches: Matches) {
  match name.as_str() {
    "load_version" => {
      println!("{:?}", matches);
      // let version = matches.args.get("version").unwrap();
      // version.value
    }
    _ => {}
  }
}

fn main() {
  tauri::Builder::default()
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
