use serde::{Deserialize, Serialize};
use std::{
  collections::BTreeMap,
  fs,
  os::windows::fs::symlink_dir,
  path::{Path, PathBuf},
};

use winapi::um::winnt::{FILE_ALL_ACCESS, PSID};
use windows_acl::{acl::ACL, helper::string_to_sid};

use crate::{model::{resolve_path_str, try_move_folder}, Error, Result};

use super::{get_game, GameObject, GameProfilesInfo, GamesRecord, ProfileInfo};

fn latest_str() -> String { "latest".to_string() }

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ProfileObject {
  #[serde(default)]
  pub iconPath: String,
  pub path: String,
  pub subfolders: BTreeMap<String, String>,
  #[serde(default = "latest_str")]
  pub version: String,
}

impl ProfileObject {
  pub fn preview_default() -> Self {
    Self {
      iconPath: String::from("/assets/preview.png"),
      path: String::from("%profiles%/preview/default"),
      subfolders: BTreeMap::new(),
      version: String::from("latest"),
    }
  }
  pub fn default() -> Self {
    Self {
      iconPath: String::from("/assets/release.png"),
      path: String::from("%profiles%/release/default"),
      subfolders: BTreeMap::new(),
      version: String::from("latest"),
    }
  }
}

impl From<&ProfileObject> for ProfileObject {
  fn from(val: &ProfileObject) -> Self {
    ProfileObject {
      iconPath: val.iconPath.clone(),
      path: val.path.clone(),
      subfolders: val.subfolders.clone(),
      version: val.version.clone(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PartialProfileObject {
  pub iconPath: Option<String>,
  pub path: Option<String>,
  pub subfolders: Option<BTreeMap<String, String>>,
  pub version: Option<String>,
}

fn remove_destination(path: &Path) {
  if path.is_symlink() {
    println!("Removing existing symbolic link");
    let rem_res = fs::remove_dir(path);
    if rem_res.is_err() {
      let err = rem_res.err();
      println!("Could not remove existing symbolic link: {:?}", err);
    }
  } else {
    let mut has_file = false;
    match fs::read_dir(path) {
      Ok(read_dir) => {
        for _ in read_dir {
          has_file = true;
        }
      }
      Err(_) => {}
    }
    if has_file {
      println!("Existing directory is empty, removing");
      let _ = fs::remove_dir(path);
    } else {
      println!("Existing directory is not a symbolic link, attempting to move");
      try_move_folder(path.to_str().unwrap(), 0);
    }
  }
}

fn apply_permissions(path: &str, sec_id: &str) {
  if sec_id == "" {
    return;
  }

  let sid = string_to_sid(sec_id).unwrap();
  let acl_res = ACL::from_file_path(path, true);
  if acl_res.is_err() {
    let err = acl_res.unwrap_err();
    println!("ACL Err: {err}");
  } else {
    let mut acl = acl_res.unwrap();
    let sid_ptr = sid.as_ptr() as PSID;
    let entry_res = acl.get(sid_ptr, None);
    if entry_res.is_ok() {
      let entries = entry_res.unwrap();
      for entry in entries {
        if entry.mask & FILE_ALL_ACCESS == FILE_ALL_ACCESS {
          println!("ACL not applied, entry already exists");
          return;
        }
      }
    }

    let allow_res = acl.allow(sid_ptr, true, FILE_ALL_ACCESS);
    if allow_res.is_err() {
      let err = allow_res.unwrap_err();
      println!("Allow Err: {err}");
    }
  }
}

pub fn create_symlink(
  data_dir: &PathBuf,
  from_str: &str,
  to_str: &str,
  sec_id: &str,
) -> Result<()> {
  let from = resolve_path_str(data_dir, from_str);
  let to = resolve_path_str(data_dir, to_str);

  if from.is_err() {
    let err = from.unwrap_err();
    println!("Failed to resolve path: {}\n{}", from_str, err);
    return Ok(());
  }
  if to.is_err() {
    let err = to.unwrap_err();
    println!("Failed to resolve path: {}\n{}", to_str, err);
    return Ok(());
  }

  let from = from.unwrap();
  let to = to.unwrap();
  let from_path = from.as_path();
  let to_path = to.as_path();

  if from_path.exists() || from_path.is_symlink() {
    println!("Existing directory found");
    remove_destination(from_path);
  }

  let from_parent = from_path.parent().unwrap();
  if !from_parent.exists() {
    if fs::create_dir_all(from_parent).is_err() {
      println!("Failed to create 'from' directory")
    };
  }

  if !to_path.exists() {
    if fs::create_dir_all(to_path).is_err() {
      println!("Failed to create 'to' directory")
    };
  }

  let sym_res = symlink_dir(to_path, from_path);
  if sym_res.is_err() {
    let err = sym_res.unwrap_err();
    return Err(Error::ProfileFailure(String::from(format!(
      "Failed to create symbolic link: {err}"
    ))));
  } else {
    println!("Created symbolic link");
  }

  apply_permissions(to_path.to_str().unwrap(), sec_id);
  apply_permissions(to_path.parent().unwrap().to_str().unwrap(), sec_id);

  println!("Applied ACL permissions");
  Ok(())
}

pub fn load_profile(data_dir: PathBuf, game: &GameObject, profile: &ProfileObject) -> Result<()> {
  let from = &game.destination;
  let to = &profile.path;
  let sec_id = &game.securityID;
  println!("'{}' -> '{}'", from, to);
  create_symlink(&data_dir, from, to, sec_id)?;

  let prof_path = Path::new(to);
  for (rel, abs) in profile.subfolders.clone() {
    if rel.ends_with("/*") {
      let substr = rel.get(..rel.len() - 2).unwrap();
      for entry in fs::read_dir(abs.clone())? {
        if let Ok(file) = entry {
          let raw_file_name = file.file_name();
          let file_name = raw_file_name.to_str().unwrap();
          let to_str = format!("{}/{}", abs, file_name);
          let file_path = file.path();
          let from_str = file_path.to_str().unwrap();
          println!("'{}/{}' -> '{}'", substr, file_name, to_str);
          create_symlink(&data_dir, from_str, &to_str, sec_id)?;
        }
      }
    } else {
      let joined = prof_path.join(rel.clone());
      let joined_str = joined.to_str().unwrap();
      println!("'{}' -> '{}'", rel, abs);
      create_symlink(&data_dir, joined_str, &abs, sec_id)?;
    }
  }
  Ok(())
}

pub fn get_profiles(record: GamesRecord, game: String) -> Result<GameProfilesInfo> {
  let name = game;
  let game = get_game(&record, name.clone())?;

  let mut profs = Vec::new();
  for (prof_name, prof) in &game.profiles {
    profs.push(ProfileInfo {
      game: name.clone(),
      name: prof_name.to_string(),
      icon: prof.iconPath.to_string(),
      version: prof.version.to_string(),
    })
  }
  Ok(GameProfilesInfo {
    game: name,
    profiles: profs,
    selected: game.selectedProfile.to_string(),
  })
}
