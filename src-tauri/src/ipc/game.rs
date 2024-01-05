use std::path::PathBuf;

use crate::{
    model::{
        self, get_games, get_profiles, resolve_path_str, version_type_str, GameProfilesInfo,
        GamesRecord, LimitedGameInfo, ProfileObject, VersionInfo,
    },
    Error,
};

use tauri::{api::dialog, command};

use super::IpcResponse;

fn dir_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let conf = &app_handle.config();
    tauri::api::path::app_data_dir(conf).unwrap()
}

fn file_path(app_handle: &tauri::AppHandle) -> PathBuf {
    dir_path(app_handle).join("games.json")
}

fn read_file(app_handle: &tauri::AppHandle) -> GamesRecord {
    model::read_games_file(file_path(app_handle))
}
fn write_file(app_handle: &tauri::AppHandle, record: &GamesRecord) {
    model::write_games_file(file_path(app_handle), record)
}

#[command]
pub fn play_game(app_handle: tauri::AppHandle, game: String) -> IpcResponse<()> {
    let data_dir: PathBuf = file_path(&app_handle);
    let file: GamesRecord = read_file(&app_handle);
    model::play_game(data_dir, file, game).into()
}

#[command]
pub fn get_full_profile(
    app_handle: tauri::AppHandle,
    game: String,
    profile: String,
) -> IpcResponse<Option<ProfileObject>> {
    let file: GamesRecord = read_file(&app_handle);
    match file.get(&game) {
        Some(obj) => {
            let prof = obj.profiles.get(&profile);
            match prof {
                Some(prof) => Ok(Option::Some(ProfileObject::from(prof))).into(),
                None => Ok(Option::None).into(),
            }
        }
        None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
    }
}

#[command]
pub fn select_profile(
    app_handle: tauri::AppHandle,
    game: String,
    profile: String,
) -> IpcResponse<()> {
    let mut file: GamesRecord = read_file(&app_handle);
    match file.get_mut(&game) {
        Some(obj) => {
            obj.selectedProfile = profile.to_string();
            write_file(&app_handle, &file);
            Ok(()).into()
        }
        None => Err(Error::GameDoesNotExist("Game does not exist".to_string())).into(),
    }
}

#[command]
pub fn list_game_profiles(
    app_handle: tauri::AppHandle,
    name: String,
) -> IpcResponse<GameProfilesInfo> {
    let file: GamesRecord = read_file(&app_handle);
    match get_profiles(file, name) {
        Ok(profs) => Ok(profs).into(),
        Err(e) => Err(e).into(),
    }
}

#[command]
pub async fn list_versions() -> IpcResponse<Vec<VersionInfo>> {
    match model::get_all_versions().await {
        Ok(vers) => {
            let infos = vers
                .into_iter()
                .map(|v| VersionInfo::new(version_type_str(v.version_type), v.version))
                .collect();
            Ok(infos).into()
        }
        Err(e) => Err(e).into(),
    }
}

#[command]
pub fn list_games(app_handle: tauri::AppHandle) -> IpcResponse<Vec<LimitedGameInfo>> {
    let res = read_file(&app_handle);
    let games = get_games(res);
    Ok(games).into()
}

#[command]
pub fn select_dir(app_handle: tauri::AppHandle, path: String) -> IpcResponse<String> {
    let data_dir: PathBuf = dir_path(&app_handle);
    let resolved = resolve_path_str(&data_dir, path.as_str()).unwrap();
    println!("{}", resolved.as_path().to_str().unwrap());
    let folder = dialog::blocking::FileDialogBuilder::default()
        .set_title("Pick Directory")
        .set_directory(
            resolved
                .parent()
                .expect("Failed to get path parent directory"),
        )
        // .set_file_name(
        //     resolved
        //         .file_name()
        //         .expect("Failed to resolve file name")
        //         .to_str()
        //         .expect("Failed to convert file name to string"),
        // )
        .pick_folder();
    let res = match &folder {
        Some(path_buf) => path_buf.to_str().unwrap(),
        None => &path,
    };
    Ok(res.to_string()).into()
}
