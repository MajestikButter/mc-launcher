use std::{fs, path::{Path, PathBuf}};
use crate::model::read_settings_file;

use crate::{Error, Result};
use crate::utils::curr_dir_path;

pub fn resolve_path_str(data_dir: &PathBuf, path: &str) -> Result<PathBuf> {
  let mut replaced = path.to_string();

  let settings = read_settings_file(data_dir.join("settings.json"));

  if replaced.contains("%versions%") {
    let versions = settings.versionsFolder.as_str();
    if versions.contains("%profiles%") || versions.contains("%versions%") {
      return Err(Error::VersionFailure(String::from("Invalid versions folder path")));
    }
    let versions_path = resolve_path_str(data_dir, versions)?;
    let versions_str = versions_path.to_str().unwrap();
    replaced = replaced.replace("%versions%", versions_str);
  }

  if replaced.contains("%profiles%") {
    let profiles = settings.profilesFolder.as_str();
    if profiles.contains("%profiles%") || profiles.contains("%versions%") {
      return Err(Error::ProfileFailure(String::from("Invalid profiles folder path")));
    }
    let profiles_path = resolve_path_str(data_dir, profiles)?;
    let profiles_str = profiles_path.to_str().unwrap();
    replaced = replaced.replace("%profiles%", profiles_str);
  }

  let base_dirs = directories::BaseDirs::new().unwrap();

  let local_app_data = base_dirs.data_local_dir();
  let local_app_data_str = local_app_data.to_str().unwrap();

  let app_data = base_dirs.data_dir();
  let app_data_str = app_data.to_str().unwrap();

  let curr_dir = curr_dir_path();
  let curr_dir_str = curr_dir.to_str().unwrap();

  replaced = replaced
    .replace("%localappdata%", local_app_data_str)
    .replace("%appdata%", app_data_str)
    .replace("%install%", curr_dir_str);

  let path = Path::new(&replaced);
  if !path.is_absolute() {
    let mut data_dir = data_dir.clone();
    if path.is_relative() {
      if path.starts_with("/") {
        let formatted = &replaced[1..];
        data_dir.push(formatted);
      } else {
        data_dir.push(path);
      }
      Ok(data_dir)
    } else {
      Ok(data_dir)
    }
  } else {
    Ok(path.to_path_buf())
  }
}

pub fn try_move_folder(from: &str, count: i32) {
  if count > 100 {
    panic!("Failed to move folder over 100 times")
  }
  if count > 0 {
    let new = format!("{from}.copy_{count}");
    if Path::new(&new).exists() {
      try_move_folder(from, count + 1);
    } else {
      let _ = fs::rename(from, new);
    }
  } else {
    let new = format!("{from}.copy");
    if Path::new(&new).exists() {
      try_move_folder(from, count + 1);
    } else {
      let _ = fs::rename(from, new);
    }
  }
}
