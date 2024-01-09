use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SettingsObject {
    #[serde(default)]
    pub keepOpen: bool,
    #[serde(default)]
    pub versionSwitching: bool,
}

fn parse_settings(str: &str) -> Result<SettingsObject> {
    let res = serde_json::from_str(str);
    match res {
        Ok(obj) => Ok(obj),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn read_settings_file(path: PathBuf) -> SettingsObject {
    let contents = fs::read_to_string(path).unwrap_or(String::new());
    let fallback = SettingsObject {
        keepOpen: true,
        versionSwitching: false,
    };
    parse_settings(&contents).unwrap_or(fallback)
}

pub fn write_settings_file(path: PathBuf, settings: &SettingsObject) {
    let contents = serde_json::to_string(settings).unwrap();
    let _ = fs::write(path, contents);
}
