use std::{fs, path::{Path, PathBuf}};

use crate::Result;

pub fn resolve_path_str(data_dir: &PathBuf, path: &str) -> Result<PathBuf> {
  let base_dirs = directories::BaseDirs::new().unwrap();
  let local_app_data = base_dirs.data_local_dir();
  let local_app_data_str = local_app_data.to_str().unwrap();
  let app_data = base_dirs.data_dir();
  let app_data_str = app_data.to_str().unwrap();
  let replaced = path
    .replace("%localappdata%", local_app_data_str)
    .replace("%appdata%", app_data_str);

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
