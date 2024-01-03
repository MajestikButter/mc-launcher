use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs};

use crate::{consts, Error, Result};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ProfileObject {
    pub iconPath: String,
    pub path: String,
    pub subfolders: BTreeMap<String, String>,
}
impl ProfileObject {
    fn default() -> Self {
        Self {
            iconPath: String::from("./unknown_icon.png"),
            path: String::from("./profiles/release/default"),
            subfolders: BTreeMap::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GameObject {
    #[serde(default)]
    pub iconPath: String,
    pub profiles: BTreeMap<String, ProfileObject>,
    pub launchScript: String,
    pub destination: String,
    pub securityID: String,
    pub selectedProfile: String,
}

impl GameObject {
    fn preview_default() -> Self {
        Self {
            iconPath: String::from("./preview.png"),
            selectedProfile: String::from("Default"),
            profiles: BTreeMap::from([(String::from("Default"), ProfileObject::default())]),
            launchScript: String::from("minecraft-preview://"),
            destination: String::from(consts::PREVIEW_DESTINATION),
            securityID: String::from(consts::PREVIEW_SECURITY_ID),
        }
    }
    fn default() -> Self {
        Self {
            iconPath: String::from("./release.png"),
            selectedProfile: String::from("Default"),
            profiles: BTreeMap::from([(String::from("Default"), ProfileObject::default())]),
            launchScript: String::from("minecraft://"),
            destination: String::from(consts::RELEASE_DESTINATION),
            securityID: String::from(consts::RELEASE_SECURITY_ID),
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

pub fn read_games_file(path: &str) -> GamesRecord {
    let contents = fs::read_to_string(path).unwrap_or(String::new());

    let fallback = BTreeMap::from([
        (String::from("Release"), GameObject::default()),
        (String::from("Preview"), GameObject::preview_default()),
    ]);
    parse_games_record(&contents).unwrap_or(fallback)
}

pub fn write_games_file(path: &str, record: &GamesRecord) {
    let contents = serde_json::to_string(record).unwrap();
    let _ = fs::write(path, contents);
}
