use std::path::PathBuf;

use tauri::command;

use crate::model::{self, SettingsObject};

use super::IpcResponse;

fn dir_path(app_handle: &tauri::AppHandle) -> PathBuf {
  let conf = &app_handle.config();
  tauri::api::path::app_data_dir(conf).unwrap()
}

fn file_path(app_handle: &tauri::AppHandle) -> PathBuf {
  dir_path(app_handle).join("settings.json")
}

fn read_file(app_handle: &tauri::AppHandle) -> SettingsObject {
  model::read_settings_file(file_path(app_handle))
}

fn write_file(app_handle: &tauri::AppHandle, record: &SettingsObject) {
  model::write_settings_file(file_path(app_handle), record)
}

#[command]
pub fn get_settings(app_handle: tauri::AppHandle) -> IpcResponse<SettingsObject> {
  let res = read_file(&app_handle);
  Ok(res).into()
}

#[command]
pub fn set_settings(app_handle: tauri::AppHandle, settings: SettingsObject) -> IpcResponse<()> {
  let res = write_file(&app_handle, &settings);
  Ok(res).into()
}
