use serde::{Deserialize, Serialize};

use crate::Result;

use super::{get_game, GamesRecord};

type VersionTuple = (String, String, i8);

#[derive(Debug, Serialize)]
pub struct Version {
    pub version: String,
    pub id: String,
    pub version_type: i8,
}
impl Version {
    pub fn latest(version_type: i8) -> Self {
        match version_type {
            2 => Self::latest_preview(),
            _ => Self::latest_release(),
        }
    }
    pub fn latest_release() -> Self {
        Self {
            version: String::from("latest"),
            id: String::from("LATEST_RELEASE"),
            version_type: 0,
        }
    }
    pub fn latest_preview() -> Self {
        Self {
            version: String::from("latest"),
            id: String::from("LATEST_PREVIEW"),
            version_type: 2,
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v: VersionTuple| Version {
            version: v.0,
            id: v.1,
            version_type: v.2,
        })
    }
}

pub async fn get_all_versions() -> Result<Vec<Version>> {
    let res = reqwest::get("https://mrarm.io/r/w10-vdb").await?;
    let json: Vec<Version> = res.json().await?;
    let reversed = json.into_iter().rev().collect();
    Ok(reversed)
}

pub async fn get_versions(version_type: i8) -> Result<Vec<Version>> {
    let versions = get_all_versions().await?;
    let collected = versions
        .into_iter()
        .filter(|ver| ver.version_type == version_type)
        .collect();
    Ok(collected)
}

pub async fn get_version(version_id: String) -> Result<Option<Version>> {
    let versions = get_all_versions().await?;
    let version = versions.into_iter().find(|v| v.version == version_id);
    Ok(version)
}

pub async fn latest_version(record: &GamesRecord, game: String) -> Result<Version> {
    let game = get_game(&record, game)?;

    let versions = get_versions(game.useVersion).await?;
    let latest = versions
        .into_iter()
        .find(|v| v.version_type == game.useVersion)
        .unwrap_or(Version::latest(game.useVersion));

    Ok(latest)
}

pub fn version_type_str(version_type: i8) -> String {
    let str = match version_type {
        0 => "release",
        2 => "preview",
        _ => "custom",
    };
    String::from(str)
}
