use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Result,
    os::windows::fs::symlink_dir,
    path::{Path, PathBuf}, collections::BTreeMap,
};

use winapi::um::winnt::{FILE_ALL_ACCESS, PSID};
use windows_acl::{acl::ACL, helper::string_to_sid};

use super::GameObject;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ProfileObject {
    pub iconPath: String,
    pub path: String,
    pub subfolders: BTreeMap<String, String>,
}
impl ProfileObject {
    pub fn preview_default() -> Self {
        Self {
            iconPath: String::from("./assets/preview.png"),
            path: String::from("./profiles/preview/default"),
            subfolders: BTreeMap::new(),
        }
    }
    pub fn default() -> Self {
        Self {
            iconPath: String::from("./assets/release.png"),
            path: String::from("./profiles/release/default"),
            subfolders: BTreeMap::new(),
        }
    }
}

fn resolve_path_str(data_dir: &PathBuf, path: &str) -> Result<PathBuf> {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let local_app_data = base_dirs.data_local_dir();
    let local_app_data_str = local_app_data.to_str().unwrap();
    let replaced = path.replace("%localappdata%", local_app_data_str);
    let path = Path::new(&replaced);
    if !path.is_absolute() {
        Ok(data_dir.join(path))
    } else {
        Ok(path.to_path_buf())
    }
}

fn remove_destination(path: &Path) {
    if path.is_symlink() {
        println!("Removing existing symbolic link");
        let rem_res = fs::remove_dir(path);
        if rem_res.is_err() {
            let err = rem_res.err();
            println!("Could not remove existing symoblic link: {:?}", err);
        }
    } else {
        println!("Existing directory is not a symbolic link, attempting to move");
        // TODO: IMPLEMENT MOVING EXISTING FILES
        panic!("Unimplemented");
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

pub fn create_symlink(data_dir: &PathBuf, from_str: &str, to_str: &str, sec_id: &str) {
    let from = resolve_path_str(data_dir, from_str);
    let to = resolve_path_str(data_dir, to_str);

    if from.is_err() {
        let err = from.unwrap_err();
        println!("Failed to resolve path: {}\n{}", from_str, err);
        return;
    }
    if to.is_err() {
        let err = to.unwrap_err();
        println!("Failed to resolve path: {}\n{}", to_str, err);
        return;
    }

    let from = from.unwrap();
    let to = to.unwrap();
    let from_path = from.as_path();
    let to_path = to.as_path();

    if from_path.exists() || from_path.is_symlink() {
        println!("Existing directory found");
        remove_destination(from_path);
    }

    if !to_path.exists() {
        if fs::create_dir_all(to_path).is_err() {
            println!("Failed to create 'to' directory")
        };
    }

    println!("Symlinking directories");
    let sym_res = symlink_dir(to_path, from_path);
    if sym_res.is_err() {
        let err = sym_res.unwrap_err();
        println!("Failed to create symbolic link: {err}");
    } else {
        println!("Created symbolic link");
    }

    apply_permissions(to_path.to_str().unwrap(), sec_id);
    apply_permissions(to_path.parent().unwrap().to_str().unwrap(), sec_id);

    println!("Applied ACL permissions");
}

pub fn load_profile(data_dir: PathBuf, game: &GameObject, profile: &ProfileObject) {
    let from = &game.destination;
    let to = &profile.path;
    let sec_id = &game.securityID;
    create_symlink(&data_dir, from, to, sec_id);

    let prof_path = Path::new(to);
    for (rel, abs) in profile.subfolders.clone() {
        let joined = prof_path.join(rel);
        let joined_str = joined.to_str().unwrap();
        create_symlink(&data_dir, joined_str, &abs, sec_id);
    }
}
