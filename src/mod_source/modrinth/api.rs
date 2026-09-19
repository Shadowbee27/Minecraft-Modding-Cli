use color_eyre::Result;
use reqwest::{StatusCode, blocking::Client};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
  config::mod_index::{ModMeta, ModSource::MODRINTH},
  mod_source::modrinth::MODRINTH_API_URL,
};

const API_EXT: &str = "v2/version_file/";

#[derive(Debug, Clone, Deserialize)]
struct VersionFile {
  game_versions: Vec<String>,
  loaders: Vec<String>,

  #[serde(rename(deserialize = "id"))]
  version_id: String,

  project_id: String,
  dependencies: Vec<Deps>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deps {
  pub version_id: Option<String>,
  pub project_id: Option<String>,
  pub file_name: Option<String>,
  pub dependency_type: DepsType,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DepsType {
  required,
  optional,
  incompatible,
  embedded,
}

pub fn get_version_from_hash<T: fmt::Display>(hash: T, mod_meta: &mut ModMeta) -> Result<()> {
  let url = format!("{}{}{}", MODRINTH_API_URL, API_EXT, hash);
  let resp = Client::new().get(url).send()?;

  if resp.status() == StatusCode::OK {
    let resp_json = resp.json::<VersionFile>()?;
    mod_meta.loaders = resp_json.loaders;
    mod_meta.mc_versions = resp_json.game_versions;
    mod_meta.project_id = resp_json.project_id;
    mod_meta.source = MODRINTH;
    mod_meta.deps = resp_json.dependencies;
    mod_meta.version_id = resp_json.version_id;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn v2_version_file() {
    get_version_from_hash(
      "72b0fcbe9d6d4bce3bc88d08ac0ee7c1df90c6b3",
      &mut ModMeta::default(),
    )
    .unwrap();
  }
}
