use color_eyre::Result;
use std::{path::PathBuf, process::Command};

pub mod download;
pub mod maven;
pub mod safe_delete;

pub fn get_current_full_dir() -> Result<PathBuf> {
  let mut cmd = Command::new("pwd");
  Ok(PathBuf::from(
    String::from_utf8(cmd.output()?.stdout)?.replace("\n", ""),
  ))
}
