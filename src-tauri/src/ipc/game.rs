use super::IpcResponse;
use std::path::PathBuf;
use regex::Regex;
use tauri::{api::dialog, command};

use crate::{
  model::{
    self, get_games, get_profiles, resolve_path_str, GameProfilesInfo,
    GamesRecord, LimitedGameInfo, ProfileObject, VersionInfo,
  },
  Error,
};
use crate::ipc::utils::curr_dir_path;
use crate::model::{BareGameObject, extract_package, get_versions_dir, PartialGameObject, PartialProfileObject};


fn file_path() -> PathBuf {
  curr_dir_path().join("games.json")
}

fn read_file() -> GamesRecord {
  model::read_games_file(file_path())
}

fn write_file(record: &GamesRecord) {
  model::write_games_file(file_path(), record)
}

#[command]
pub async fn play_game(
  app_handle: tauri::AppHandle,
  game: String,
  with_version: bool,
) -> IpcResponse<()> {
  let data_dir: PathBuf = curr_dir_path();
  let file: GamesRecord = read_file();
  let req_path = app_handle.path_resolver().resolve_resource("./resources/download_request.xml").unwrap();
  match model::play_game(data_dir, req_path, file, game, with_version).await {
    Ok(_) => Ok(()).into(),
    Err(e) => Err(e).into(),
  }
}

#[command]
pub fn get_full_profile(
  game: String,
  profile: String,
) -> IpcResponse<ProfileObject> {
  let file: GamesRecord = read_file();
  match file.get(&game) {
    Some(obj) => {
      let prof = obj.profiles.get(&profile);
      match prof {
        Some(prof) => Ok(ProfileObject::from(prof)).into(),
        None => Err(Error::ProfileFailure("Profile does not exist".to_string())).into(),
      }
    }
    None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
  }
}

#[command]
pub fn select_profile(
  game: String,
  profile: String,
) -> IpcResponse<()> {
  let mut file: GamesRecord = read_file();
  match file.get_mut(&game) {
    Some(obj) => {
      obj.selectedProfile = profile.to_string();
      write_file(&file);
      Ok(()).into()
    }
    None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
  }
}

#[command]
pub fn update_profile(
  game: String,
  profile: String,
  data: PartialProfileObject,
) -> IpcResponse<()> {
  let mut file: GamesRecord = read_file();
  match file.get_mut(&game) {
    Some(obj) => match obj.profiles.get_mut(&profile) {
      None => return Ok(()).into(),
      Some(prof) => {
        if let Some(icon) = data.iconPath { prof.iconPath = icon; }
        if let Some(path) = data.path { prof.path = path; }
        if let Some(subfolders) = data.subfolders { prof.subfolders = subfolders; }
        if let Some(version) = data.version { prof.version = version; }
        write_file(&file);
        Ok(()).into()
      }
    },
    None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
  }
}

#[command]
pub fn list_game_profiles(
  name: String,
) -> IpcResponse<GameProfilesInfo> {
  let file: GamesRecord = read_file();
  match get_profiles(file, name) {
    Ok(profs) => Ok(profs).into(),
    Err(e) => Err(e).into(),
  }
}

#[command]
pub fn get_full_game(
  game: String,
) -> IpcResponse<BareGameObject> {
  let file: GamesRecord = read_file();
  match file.get(&game) {
    Some(obj) => Ok(obj.into()).into(),
    None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
  }
}

#[command]
pub fn update_game(
  game: String,
  data: PartialGameObject,
) -> IpcResponse<()> {
  let mut file: GamesRecord = read_file();
  match file.get_mut(&game) {
    Some(obj) => {
      if let Some(icon) = data.iconPath { obj.iconPath = icon }
      if let Some(path) = data.destination { obj.destination = path; }
      if let Some(background) = data.backgroundPath { obj.backgroundPath = background; }
      if let Some(ver) = data.useVersion { obj.useVersion = ver; }
      if let Some(launch) = data.launchScript { obj.launchScript = launch; }
      write_file(&file);
      Ok(()).into()
    }
    None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
  }
}

#[command]
pub async fn list_versions() -> IpcResponse<Vec<VersionInfo>> {
  let data_dir = curr_dir_path();
  match model::get_all_versions(data_dir.clone()).await {
    Ok(vers) => {
      let infos = vers
        .into_iter()
        .map(|v| {
          VersionInfo::new(
            v.version_type,
            v.version.clone(),
            v.installed(data_dir.clone()),
          )
        })
        .collect();
      Ok(infos).into()
    }
    Err(e) => Err(e).into(),
  }
}

#[command]
pub fn list_games() -> IpcResponse<Vec<LimitedGameInfo>> {
  let res = read_file();
  let games = get_games(res);
  Ok(games).into()
}

#[command]
pub fn select_dir(path: String) -> IpcResponse<String> {
  let data_dir: PathBuf = curr_dir_path();
  let resolved = resolve_path_str(&data_dir, path.as_str()).unwrap();
  let folder = dialog::blocking::FileDialogBuilder::new()
    .set_title("Pick Directory")
    .set_directory(resolved)
    .pick_folder();
  let res = match &folder {
    Some(path_buf) => path_buf.to_str().unwrap(),
    None => &path,
  };
  Ok(res.to_string()).into()
}


#[command]
pub fn import_version() -> IpcResponse<()> {
  let data_dir: PathBuf = curr_dir_path();
  let folder = dialog::blocking::FileDialogBuilder::new()
    .set_title("Import Version")
    .add_filter("*", &["appx"])
    .pick_file();
  if let Some(path_buf) = folder {
    let re = Regex::new(r"[\d.]+").unwrap();
    let raw_name = path_buf.file_name().unwrap().to_str().unwrap();
    let name = re.find(raw_name).unwrap().as_str();
    let dir = format!("custom/{}", name);
    let versions_dir = get_versions_dir(data_dir).unwrap();
    let destination = versions_dir.join(dir);
    let _ = extract_package(path_buf, destination);
  }
  Ok(()).into()
}
