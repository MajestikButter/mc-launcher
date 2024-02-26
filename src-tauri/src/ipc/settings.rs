use std::path::PathBuf;

use tauri::command;
use crate::ipc::utils::curr_dir_path;

use crate::model::{self, SettingsObject};

use super::IpcResponse;


fn file_path() -> PathBuf {
  curr_dir_path().join("settings.json")
}

fn read_file() -> SettingsObject {
  model::read_settings_file(file_path())
}

fn write_file(record: &SettingsObject) {
  model::write_settings_file(file_path(), record)
}

#[command]
pub fn get_settings() -> IpcResponse<SettingsObject> {
  let res = read_file();
  Ok(res).into()
}

#[command]
pub fn set_settings(settings: SettingsObject) -> IpcResponse<()> {
  let res = write_file(&settings);
  Ok(res).into()
}
