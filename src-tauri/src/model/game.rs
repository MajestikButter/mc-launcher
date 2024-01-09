use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};

use crate::{consts, Error, Result};

use super::{
    download_version, version_type_str, LimitedGameInfo,
    ProfileObject, Version,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GameObject {
    #[serde(default)]
    pub backgroundPath: String,
    #[serde(default)]
    pub iconPath: String,
    pub profiles: BTreeMap<String, ProfileObject>,
    pub launchScript: String,
    pub destination: String,
    pub securityID: String,
    pub selectedProfile: String,
    pub useVersion: i8,
}

impl GameObject {
    pub fn preview_default() -> Self {
        Self {
            backgroundPath: String::from("/assets/preview.png"),
            iconPath: String::from("/assets/preview.png"),
            selectedProfile: String::from("Default"),
            profiles: BTreeMap::from([(String::from("Default"), ProfileObject::preview_default())]),
            launchScript: String::from("minecraft-preview://"),
            destination: String::from(consts::PREVIEW_DESTINATION),
            securityID: String::from(consts::PREVIEW_SECURITY_ID),
            useVersion: 2,
        }
    }
    pub fn default() -> Self {
        Self {
            backgroundPath: String::from("/assets/release.png"),
            iconPath: String::from("/assets/release.png"),
            selectedProfile: String::from("Default"),
            profiles: BTreeMap::from([(String::from("Default"), ProfileObject::default())]),
            launchScript: String::from("minecraft://"),
            destination: String::from(consts::RELEASE_DESTINATION),
            securityID: String::from(consts::RELEASE_SECURITY_ID),
            useVersion: 0,
        }
    }
}

pub type GamesRecord = BTreeMap<String, GameObject>;

fn parse_games_record(str: &str) -> Result<GamesRecord> {
    let res = serde_json::from_str(str);
    match res {
        Ok(obj) => Ok(obj),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn read_games_file(path: PathBuf) -> GamesRecord {
    let contents = fs::read_to_string(path).unwrap_or(String::new());

    let fallback = BTreeMap::from([
        (String::from("Release"), GameObject::default()),
        (String::from("Preview"), GameObject::preview_default()),
    ]);
    parse_games_record(&contents).unwrap_or(fallback)
}

pub fn write_games_file(path: PathBuf, record: &GamesRecord) {
    let contents = serde_json::to_string(record).unwrap();
    let _ = fs::write(path, contents);
}

pub fn get_game(record: &GamesRecord, game: String) -> Result<&GameObject> {
    match record.get(&game) {
        Some(game) => Ok(game),
        None => Err(Error::GameDoesNotExist("Game does not exist".to_string())),
    }
}

pub fn get_games(record: GamesRecord) -> Vec<LimitedGameInfo> {
    let mut vec: Vec<LimitedGameInfo> = Vec::new();
    for (name, obj) in record {
        vec.push(LimitedGameInfo {
            name: name,
            icon: obj.iconPath,
            background: obj.backgroundPath,
            versionType: version_type_str(obj.useVersion),
        })
    }
    vec
}

pub async fn play_game(
    data_dir: PathBuf,
    record: GamesRecord,
    game: String,
    with_version: bool,
) -> Result<()> {
    let game = get_game(&record, game)?;
    let prof = game.profiles.get(&game.selectedProfile);
    if prof.is_some() {
        let prof = prof.unwrap();

        // if with_version && (game.useVersion == 0 || game.useVersion == 2) {
        //     if prof.version == "latest" {
        let ver = Version::latest(game.useVersion);
        download_version(ver.id, "1".to_owned(), String::new()).await;
        // load_version(data_dir.clone(), ver)?;
        //     } else {
        //         let version = get_version(prof.version.clone()).await?;
        //         match version {
        //             Some(version) => {
        //                 load_version(data_dir.clone(), version)?;
        //             }
        //             None => {
        //                 return Err(Error::VersionDoesNotExist(String::from(
        //                     "Version does not exist",
        //                 )));
        //             }
        //         }
        //     }
        // }

        // load_profile(data_dir, game, prof)?;
        // Command::new("rundll32.exe")
        //     .arg("url.dll,FileProtocolHandler")
        //     .arg(&game.launchScript)
        //     .output()
        //     .expect("Failed to launch game");
    }
    Ok(())
}
