use std::path::PathBuf;

pub fn curr_dir_path() -> PathBuf {
  std::env::current_exe().unwrap().parent().unwrap().to_path_buf()
}
