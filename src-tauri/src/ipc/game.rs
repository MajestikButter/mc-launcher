use crate::{
    model::{self, GameObject, GamesRecord},
    Error,
};

use serde::Serialize;
use tauri::{command, Manager};

use super::IpcResponse;

fn read_file() -> GamesRecord {
    model::read_games_file("./data/games.json")
}
fn write_file(record: &GamesRecord) {
    model::write_games_file("./data/games.json", record)
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
}

#[command]
pub fn select_profile(game: String, profile: String) -> IpcResponse<()> {
    let mut file = read_file();
    let obj = file.get_mut(&game);

    let res = match obj {
        None => Err(Error::GameDoesNotExist(format!(
            "no game named '{game}' exists"
        ))),
        Some(obj) => {
            obj.selectedProfile = profile.to_string();
            write_file(&file);
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
    let file = read_file();
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
    let res = read_file();
    let mut vec: Vec<LimitedGameInfo> = Vec::new();
    for (name, obj) in res {
        vec.push(LimitedGameInfo {
            name: name,
            icon: obj.iconPath,
        })
    }
    let _ = app_handle.emit_all("mc-launcher://update-games", &vec);
    IpcResponse::from(Ok(vec))
}
