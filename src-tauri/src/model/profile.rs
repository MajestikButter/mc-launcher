use std::os::windows::fs::symlink_dir;

use winapi::um::winnt::{FILE_ALL_ACCESS, PSID};
use windows_acl::{acl::ACL, helper::string_to_sid};

pub fn create_symlink(from: &str, to: &str, sec_id: &str) {
    let _ = symlink_dir(to, from);

    let sid = string_to_sid(sec_id).unwrap();
    let acl_res = ACL::from_file_path(to, true);
    if acl_res.is_err() {
        let err = acl_res.unwrap_err();
        println!("Err: {err}");
    } else {
        let mut acl = acl_res.unwrap();
        let sid_ptr = sid.as_ptr() as PSID;
        let _ = acl.allow(sid_ptr, true, FILE_ALL_ACCESS);
    }
}
