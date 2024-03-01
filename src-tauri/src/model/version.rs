use std::{fmt::Debug, fs, path::PathBuf};
use std::fs::File;
use log::{debug, info};

use serde::{Deserialize, Serialize};
use windows::{
  core::HSTRING,
  ApplicationModel::Package,
  Foundation::{Collections::IIterable, Uri},
  Management::Deployment::{DeploymentOptions, PackageManager},
};

use crate::{consts, Error, Result};

use super::{download_version, read_settings_file, resolve_path_str};

type VersionTuple = (String, String, i8);

#[derive(Debug, Serialize)]
pub struct Version {
  pub version: String,
  pub id: String,
  pub version_type: i8,
  dir: String,
}

impl Version {
  pub fn installed(&self, versions_dir: PathBuf) -> bool {
    let path = &self.game_directory(versions_dir);
    match path {
      Ok(path) => path.exists(),
      Err(_) => false,
    }
  }

  pub fn game_directory(&self, versions_dir: PathBuf) -> Result<PathBuf> {
    let dir = match self.version_type {
      0 => {
        format!("Release-{}", self.version)
      }
      2 => {
        format!("Preview-{}", self.version)
      }
      _ => self.dir.clone(),
    };
    Ok(versions_dir.join(&dir))
  }

  pub fn package_family(&self) -> &str {
    if self.version_type == 2 {
      consts::MINECRAFT_PREVIEW
    } else {
      consts::MINECRAFT
    }
  }
}

impl Clone for Version {
  fn clone(&self) -> Self {
    Version { id: self.id.clone(), version: self.version.clone(), dir: self.dir.clone(), version_type: self.version_type.clone() }
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

pub fn get_versions_dir(data_dir: PathBuf) -> Result<PathBuf> {
  resolve_path_str(&data_dir, &read_settings_file(data_dir.join("settings.json")).versionsFolder)
}

pub fn version_from_dir(path: PathBuf) -> Version {
  Version {
    id: "CUSTOM".to_string(),
    dir: path.to_str().unwrap().to_owned(),
    version: path.file_name().unwrap().to_str().unwrap().to_owned(),
    version_type: 3,
  }
}

pub async fn get_all_versions(data_dir: PathBuf) -> Result<Vec<Version>> {
  debug!("Getting all versions");

  let versions_dir = get_versions_dir(data_dir)?;
  let version_cache = versions_dir.join("version_cache.json");

  let res = reqwest::get("https://mrarm.io/r/w10-vdb").await;
  let mut reversed: Vec<Version> = Vec::new();
  if let Ok(res) = res {
    let json: reqwest::Result<Vec<Version>> = res.json().await;
    if let Ok(json) = json {
      reversed = json
        .into_iter()
        .filter(|ver| ver.version_type != 1)
        .rev()
        .collect();

      let contents = serde_json::to_string(&reversed).unwrap();
      let _ = fs::write(version_cache, contents);
    } else if let Ok(contents) = fs::read_to_string(version_cache) {
      reversed = serde_json::from_str(&contents)?;
    }
  }

  let base_custom = versions_dir.join("custom");
  if let Ok(custom) = fs::read_dir(base_custom) {
    for entry in custom {
      if let Ok(file) = entry {
        let path = file.path();
        if path.is_dir() {
          reversed.push(version_from_dir(path));
        }
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
  use_version: i8,
) -> Result<Version> {
  let versions = get_versions(data_dir, use_version).await?;
  let latest = (*versions.first().unwrap()).clone();
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

fn register_package(family: &str, game_dir: PathBuf) -> Result<()> {
  info!("Family: {}", family);
  let packages = get_package_iter(family)?;

  info!("Got package iterator");

  for pkg in packages {
    let pkg_dir = pkg.InstalledPath().unwrap_or_default().to_string();
    if pkg_dir == game_dir.to_str().unwrap() {
      info!("Skipping register");
      return Ok(());
    }
    info!("Removing package");
    remove_package(&pkg)?;
    info!("Removed package");
  }

  let manifest_path = game_dir.join("AppxManifest.xml");
  let manifest_str = manifest_path.to_str().unwrap();

  let manifesturi = &Uri::CreateUri(&HSTRING::from(manifest_str))?;
  let dependencypackageuris = None;
  let deploymentoptions = DeploymentOptions::DevelopmentMode;

  info!("Registering package");
  let res = manager()
    .RegisterPackageAsync(manifesturi, dependencypackageuris, deploymentoptions)?
    .get()?;

  if !res.IsRegistered().unwrap_or(false) {
    return Err(Error::VersionFailure(String::from(format!(
      "Failed to register package {}",
      res.ErrorText().unwrap_or_default()
    ))));
  }
  info!("Registered package");

  Ok(())
}

pub fn extract_package(
  zip_path: PathBuf,
  destination: PathBuf,
) -> Result<()> {
  let zip_file = File::open(zip_path.clone())?;
  let mut archive = zip::ZipArchive::new(zip_file)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let out_path = match file.enclosed_name() {
      Some(path) => {
        let path_str = path.to_str().unwrap();
        if consts::EXCLUDE_IN_PACKAGE.iter().find(|v| { (**v).eq(path_str) }).is_some() {
          continue;
        }
        destination.join(path.to_owned())
      }
      None => continue,
    };

    if file.name().ends_with("/") {
      fs::create_dir_all(out_path)?;
    } else {
      if let Some(p) = out_path.parent() {
        if !p.exists() {
          fs::create_dir_all(p).unwrap();
        }
      }

      let mut outfile = File::create(out_path)?;
      std::io::copy(&mut file, &mut outfile)?;
    }
  }

  Ok(())
}

pub fn extract_version(
  data_dir: PathBuf,
  version: Version,
  remove_archive: bool,
) -> Result<()> {
  info!("Extracting version");
  let versions_dir = get_versions_dir(data_dir.clone())?;
  let file_name = format!("Minecraft-{}.appx", version.version);
  let zip_path = versions_dir.join(PathBuf::from(file_name));
  let destination = version.game_directory(versions_dir)?;

  extract_package(zip_path.clone(), destination)?;

  if remove_archive {
    fs::remove_file(zip_path)?;
  }
  info!("Extracted version");
  Ok(())
}

pub async fn load_version(data_dir: PathBuf, req_path: PathBuf, ver: Version) -> Result<()> {
  let versions_dir = get_versions_dir(data_dir.clone())?;
  if !ver.installed(versions_dir.clone()) {
    download_version(data_dir.clone(), req_path, ver.id.clone(), "1".to_owned(), ver.version.clone()).await?;
    extract_version(data_dir.clone(), ver.clone(), true)?;
  }
  let game_dir = ver.game_directory(versions_dir)?;

  register_package(ver.package_family(), game_dir)?;
  Ok(())
}
