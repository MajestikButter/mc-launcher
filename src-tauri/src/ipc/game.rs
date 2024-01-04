use std::{path::PathBuf, process::Command};

use crate::{
    model::{self, load_profile, GamesRecord},
    Error,
};

use serde::Serialize;
use tauri::{command, Manager};

use super::IpcResponse;

fn file_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let conf = &app_handle.config();
    tauri::api::path::app_data_dir(conf).unwrap().join("games.json")
}

fn read_file(app_handle: &tauri::AppHandle) -> GamesRecord {
    model::read_games_file(file_path(app_handle))
}
fn write_file(app_handle: &tauri::AppHandle, record: &GamesRecord) {
    model::write_games_file(file_path(app_handle), record)
}

#[derive(Serialize)]
pub struct ProfileInfo {
    game: String,
    name: String,
    icon: String,
}
#[derive(Serialize)]
pub struct GameProfilesInfo {
    game: String,
    profiles: Vec<ProfileInfo>,
    selected: String,
}
#[derive(Serialize)]
pub struct LimitedGameInfo {
    name: String,
    icon: String,
    background: String,
}

#[command]
pub fn play_game(app_handle: tauri::AppHandle, game: String) -> IpcResponse<()> {
    let file = read_file(&app_handle);
    let obj = file.get(&game);

    let res = match obj {
        None => Err(Error::GameDoesNotExist(format!(
            "no game named '{game}' exists"
        ))),
        Some(obj) => {
            let prof = obj.profiles.get(&obj.selectedProfile).unwrap();
            load_profile(file_path(&app_handle), obj, prof);
            Command::new("rundll32.exe")
                .arg("url.dll,FileProtocolHandler")
                .arg(&obj.launchScript)
                .output()
                .expect("Failed to launch game");
            Ok(())
        }
    };
    IpcResponse::from(res)
}

#[command]
pub fn select_profile(
    app_handle: tauri::AppHandle,
    game: String,
    profile: String,
) -> IpcResponse<()> {
    let mut file = read_file(&app_handle);
    let obj = file.get_mut(&game);

    let res = match obj {
        None => Err(Error::GameDoesNotExist(format!(
            "no game named '{game}' exists"
        ))),
        Some(obj) => {
            obj.selectedProfile = profile.to_string();
            write_file(&app_handle, &file);
            Ok(())
        }
    };
    IpcResponse::from(res)
}

#[command]
pub fn request_profiles_update(
    app_handle: tauri::AppHandle,
    name: String,
) -> IpcResponse<GameProfilesInfo> {
    let file = read_file(&app_handle);
    let obj = file.get(&name);

    let res = match obj {
        None => Err(Error::GameDoesNotExist(format!(
            "no game named '{name}' exists"
        ))),
        Some(obj) => {
            let mut profs = Vec::new();
            for (prof_name, prof) in &obj.profiles {
                profs.push(ProfileInfo {
                    game: name.clone(),
                    name: prof_name.to_string(),
                    icon: prof.iconPath.to_string(),
                })
            }
            let info = GameProfilesInfo {
                game: name,
                profiles: profs,
                selected: obj.selectedProfile.to_string(),
            };
            let _ = app_handle.emit_all("mc-launcher://update-profiles", &info);
            Ok(info)
        }
    };
    IpcResponse::from(res)
}

#[command]
pub fn request_games_update(app_handle: tauri::AppHandle) -> IpcResponse<Vec<LimitedGameInfo>> {
    let res = read_file(&app_handle);
    let mut vec: Vec<LimitedGameInfo> = Vec::new();
    for (name, obj) in res {
        vec.push(LimitedGameInfo {
            name: name,
            icon: obj.iconPath,
            background: obj.backgroundPath,
        })
    }
    let _ = app_handle.emit_all("mc-launcher://update-games", &vec);
    IpcResponse::from(Ok(vec))
}
