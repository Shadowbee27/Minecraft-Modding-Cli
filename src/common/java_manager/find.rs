use color_eyre::Result;
use std::{fs::exists, path::PathBuf};

use crate::common::java_manager::java_paths::{JavaPath, get_possible_paths};

use super::JavaVersion;
use super::java_paths::JavaType;

pub fn find_javas() -> Result<Vec<JavaVersion>> {
  let java_paths = get_possible_paths();
  let _check_paths = scan(&java_paths)?;
  unimplemented!()
}

pub fn scan(possible_paths: &Vec<JavaPath>) -> Result<Vec<PathBuf>> {
  let mut existing_paths = Vec::new();

  for path in possible_paths {
    match &path.java_type {
      JavaType::BIN => {
        if exists(&path.path)? {
          existing_paths.push(path.path.clone());
        }
      }
      JavaType::SCANPATH(_f) => {}
      #[cfg(target_os = "windows")]
      JavaType::REGISTRY => {}
    }
  }

  Ok(existing_paths)
}
