use color_eyre::{
  Result,
  eyre::{ContextCompat, eyre},
};
use rayon::prelude::*;
use std::{path::PathBuf, process::Command};

use crate::common::java_manager::JavaVersion;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct JavaInfo {
  pub java_runtime_version: String,
  pub java_vendor: String,
  pub os_arch: String,
  pub path: PathBuf,
}

pub fn check_javas(javas: Vec<PathBuf>) -> Vec<JavaVersion> {
  let java_infos: Vec<JavaInfo> = javas
    .par_iter()
    .filter_map(|p| get_jvm_info(p).ok())
    .collect();

  java_infos
    .iter()
    .filter_map(|i| JavaVersion::from_java_info(i).ok())
    .collect()
}

fn get_jvm_info(java_path: &PathBuf) -> Result<JavaInfo> {
  let mut java_cmd = Command::new(&java_path);

  let mut java_info = JavaInfo::default();
  java_info.path = java_path.to_owned();

  match java_cmd
    .args(["-XshowSettings:properties", "-version"])
    .output()
  {
    Ok(o) => {
      if !o.status.success() {
        eprintln!("Warning: Failed to check Java at {:?}", java_path);
        return Err(eyre!(""));
      }

      let output = String::from_utf8_lossy(&o.stderr);

      for line in output.lines() {
        if line.contains("java.runtime.version") && java_info.java_runtime_version.is_empty() {
          let (_, java_runtime_version) = line
            .split_once("=")
            .wrap_err("Failed to parse java.runtime.version")?;

          java_info.java_runtime_version = java_runtime_version.trim().to_string();
        } else if line.contains("java.vendor") && java_info.java_vendor.is_empty() {
          let (_, java_vendor) = line
            .split_once("=")
            .wrap_err("Failed to parse java.runtime.version")?;

          java_info.java_vendor = java_vendor.trim().to_string();
        } else if line.contains("os.arch") && java_info.os_arch.is_empty() {
          let (_, os_arch) = line
            .split_once("=")
            .wrap_err("Failed to parse java.runtime.version")?;

          java_info.os_arch = os_arch.trim().to_string();
        }
      }
    }
    Err(e) => {
      eprintln!(
        "Warning: Failed to check Java at {:?}, Error: {e}",
        java_path
      );
      return Err(eyre!(e));
    }
  }

  Ok(java_info)
}
