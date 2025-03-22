use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

use crate::{consts, Error, Result};

use super::{get_version, load_profile, load_version, LimitedGameInfo, ProfileObject, VersionType, latest_version};

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
  #[serde(default)]
  pub useVersion: VersionType,
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BareGameObject {
  pub backgroundPath: String,
  pub iconPath: String,
  pub launchScript: String,
  pub destination: String,
  pub useVersion: i8,
}

impl From<&GameObject> for BareGameObject {
  fn from(val: &GameObject) -> Self {
    BareGameObject {
      backgroundPath: val.backgroundPath.clone(),
      iconPath: val.iconPath.clone(),
      launchScript: val.launchScript.clone(),
      destination: val.destination.clone(),
      useVersion: val.useVersion.clone(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PartialGameObject {
  pub backgroundPath: Option<String>,
  pub iconPath: Option<String>,
  pub launchScript: Option<String>,
  pub destination: Option<String>,
  pub useVersion: Option<i8>,
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
      versionType: obj.useVersion,
    })
  }
  vec
}

pub async fn play_game(
  data_dir: PathBuf,
  req_path: PathBuf,
  record: GamesRecord,
  game: String,
  with_version: bool,
) -> Result<()> {
  let game = get_game(&record, game)?;
  let prof = game.profiles.get(&game.selectedProfile);
  if prof.is_some() {
    let prof = prof.unwrap();

    if with_version && (game.useVersion == 0 || game.useVersion == 2 || game.useVersion == 3) {
      if prof.version == "latest" && game.useVersion != 3 {
        let latest_ver = latest_version(data_dir.clone(), game.useVersion).await?;
        load_version(data_dir.clone(), req_path.clone(), latest_ver).await?;
      } else {
        let version = get_version(data_dir.clone(), prof.version.clone()).await?;
        match version {
          Some(version) => {
            load_version(data_dir.clone(), req_path.clone(), version).await?;
          }
          None => {
            return Err(Error::VersionDoesNotExist(String::from(
              "Version does not exist",
            )));
          }
        }
      }
    }

    load_profile(data_dir, game, prof)?;
    Command::new("rundll32.exe")
      .arg("url.dll,FileProtocolHandler")
      .arg(&game.launchScript)
      .output()
      .expect("Failed to launch game");
  }
  Ok(())
}
