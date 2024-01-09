use std::{fmt::Debug, fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use windows::{
    core::HSTRING,
    ApplicationModel::Package,
    Foundation::{Collections::IIterable, Uri},
    Management::Deployment::{DeploymentOptions, PackageManager},
};

use crate::{consts, Error, Result};

use super::{get_game, GamesRecord};

type VersionTuple = (String, String, i8);

#[derive(Debug, Serialize)]
pub struct Version {
    pub version: String,
    pub id: String,
    pub version_type: i8,
    dir: String,
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
            dir: String::new(),
        }
    }
    pub fn latest_preview() -> Self {
        Self {
            version: String::from("latest"),
            id: String::from("LATEST_PREVIEW"),
            version_type: 2,
            dir: String::new(),
        }
    }

    pub fn installed(&self, data_dir: PathBuf) -> bool {
        let path = &self.game_directory(data_dir);
        path.exists()
    }

    pub fn game_directory(&self, data_dir: PathBuf) -> PathBuf {
        let dir = match self.version_type {
            0 => {
                let mut str = "Release-".to_owned();
                str.push_str(&self.version);
                str
            }
            2 => {
                let mut str = "Preview-".to_owned();
                str.push_str(&self.version);
                str
            }
            _ => self.dir.clone(),
        };
        data_dir.join("versions").join(&dir)
    }

    pub fn package_family(&self) -> &str {
        if self.version_type == 2 {
            consts::MINECRAFT_PREVIEW
        } else {
            consts::MINECRAFT
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
            dir: String::new(),
        })
    }
}

pub async fn get_all_versions(data_dir: PathBuf) -> Result<Vec<Version>> {
    let res = reqwest::get("https://mrarm.io/r/w10-vdb").await?;
    let json: Vec<Version> = res.json().await?;
    let mut reversed: Vec<Version> = json
        .into_iter()
        .filter(|ver| ver.version_type != 1)
        .rev()
        .collect();

    let base_custom = data_dir.join("versions").join("custom");
    let custom = fs::read_dir(base_custom);
    if custom.is_ok() {
        for entry in custom.unwrap() {
            match entry {
                Ok(file) => {
                    let path = file.path();

                    if path.is_dir() {
                        reversed.push(Version {
                            id: "CUSTOM".to_string(),
                            dir: path.to_str().unwrap().to_owned(),
                            version: path.file_name().unwrap().to_str().unwrap().to_owned(),
                            version_type: 3,
                        });
                    }
                }
                Err(_) => {}
            }
        }
    }

    Ok(reversed)
}

pub async fn get_versions(data_dir: PathBuf, version_type: i8) -> Result<Vec<Version>> {
    let versions = get_all_versions(data_dir).await?;
    let collected = versions
        .into_iter()
        .filter(|ver| ver.version_type == version_type)
        .collect();
    Ok(collected)
}

pub async fn get_version(data_dir: PathBuf, version_id: String) -> Result<Option<Version>> {
    let versions = get_all_versions(data_dir).await?;
    let version = versions.into_iter().find(|v| v.version == version_id);
    Ok(version)
}

pub async fn latest_version(
    data_dir: PathBuf,
    record: &GamesRecord,
    game: String,
) -> Result<Version> {
    let game = get_game(&record, game)?;

    let versions = get_versions(data_dir, game.useVersion).await?;
    let latest = versions
        .into_iter()
        .find(|v| v.version_type == game.useVersion)
        .unwrap_or(Version::latest(game.useVersion));

    Ok(latest)
}

fn manager() -> PackageManager {
    PackageManager::new().unwrap()
}

fn remove_package(pkg: &Package) -> Result<()> {
    let packagefullname = &pkg.Id()?.FullName()?;
    manager().RemovePackageAsync(packagefullname)?.get()?;
    Ok(())
}

fn get_package_iter(family: &str) -> Result<IIterable<Package>> {
    let packagefamilyname = &HSTRING::from(family);
    Ok(manager().FindPackagesByPackageFamilyName(packagefamilyname)?)
}

fn reregister_package(family: &str, game_dir: PathBuf) -> Result<()> {
    let packages = get_package_iter(family)?;

    for pkg in packages {
        let pkg_dir = pkg.InstalledPath().unwrap_or_default().to_string();
        if pkg_dir == game_dir.to_str().unwrap() {
            println!("Skipping reregister");
            return Ok(());
        }
        println!("Removing package");
        remove_package(&pkg)?;
        println!("Removed package");
    }

    let manifest_path = game_dir.join("AppxManifest.xml");
    let manifest_str = manifest_path.to_str().unwrap();

    let manifesturi = &Uri::CreateUri(&HSTRING::from(manifest_str))?;
    let dependencypackageuris = None;
    let deploymentoptions = DeploymentOptions::DevelopmentMode;

    println!("Registering package");
    let res = manager()
        .RegisterPackageAsync(manifesturi, dependencypackageuris, deploymentoptions)?
        .get()?;

    if !res.IsRegistered().unwrap_or(false) {
        return Err(Error::VersionFailure(String::from(format!(
            "Failed to register package {}",
            res.ErrorText().unwrap_or_default()
        ))));
    }
    println!("Registered package");

    Ok(())
}

fn unregister_package(family: &str, game_dir: String) -> Result<()> {
    let packages = get_package_iter(family)?;
    for pkg in packages {
        match pkg.InstalledPath() {
            Ok(path) => {
                if path.to_string() == game_dir {
                    remove_package(&pkg)?;
                }
            }
            Err(_e) => {
                remove_package(&pkg)?;
            }
        }
    }
    Ok(())
}

pub fn load_version(data_dir: PathBuf, ver: Version) -> Result<()> {
    let game_dir = ver.game_directory(data_dir);
    println!("register ver {}", ver.version);
    reregister_package(ver.package_family(), game_dir)?;
    println!("registered ver {}", ver.version);

    Ok(())
}

pub fn version_type_str(version_type: i8) -> String {
    let str = match version_type {
        0 => "release",
        2 => "preview",
        _ => "custom",
    };
    String::from(str)
}
