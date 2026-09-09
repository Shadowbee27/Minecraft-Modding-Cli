use color_eyre::Result;
use color_eyre::eyre::Ok;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
  fs::{self, create_dir_all, exists},
  path::PathBuf,
};

use crate::mod_source::modrinth::api::Deps;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModsIndex {
  pub path: PathBuf,
  pub version: Version,
  pub mod_meta: Option<Vec<ModMeta>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModMeta {
  #[serde(default)]
  pub version_id: String,
  pub source: ModSource,
  pub hash: String,
  pub project_id: String,
  pub loaders: Vec<String>,
  pub mc_versions: Vec<String>,
  #[serde(default)]
  pub deps: Vec<Deps>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ModSource {
  // CURSEFORGE,
  MODRINTH,
  #[default]
  UNKNOWN,
}

impl ModsIndex {
  pub fn new(folder: PathBuf) -> Result<ModsIndex> {
    let index = ModsIndex {
      path: folder.clone(),
      ..Default::default()
    };

    if !exists(&folder)? {
      create_dir_all(folder)?;
    }

    index.write_conf()?;
    Ok(index)
  }

  /// This expects the folder to exist
  pub fn write_conf(&self) -> Result<()> {
    fs::write(
      self.path.join(".mcmc-index/mods.json"),
      serde_json::to_string(self)?,
    )?;
    Ok(())
  }
}

impl Default for ModsIndex {
  fn default() -> Self {
    ModsIndex {
      path: PathBuf::new(),
      mod_meta: None,
      version: Version::parse("0.0.0").unwrap(),
    }
  }
}
