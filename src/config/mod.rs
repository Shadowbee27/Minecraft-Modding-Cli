use color_eyre::{
  Result,
  eyre::{Context, eyre},
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
  fs::{self, exists},
  path::{Path, PathBuf},
};

use crate::config::project_conf::PROJECT_CONFIG_PATH;

use super::software::{McVersion, ServerSoftware};

pub mod global_conf;
pub mod mod_index;
pub mod project_conf;

#[derive(Deserialize, Serialize)]
pub struct ProjectConfig {
  pub config_version: Version,
  pub flavor: ServerSoftware,
  pub flavor_version: String,
  pub mc_version: McVersion,
  pub jar_file: String,
}

impl ProjectConfig {
  pub fn save(&self, project_dir: &Path) -> Result<()> {
    let serialized = toml::to_string(&self)?;

    fs::write(project_dir.join(PROJECT_CONFIG_PATH), serialized)?;
    Ok(())
  }

  pub fn load_config(path_prefix: &Option<PathBuf>) -> Result<ProjectConfig> {
    let conf_path = if let Some(prefix) = path_prefix {
      prefix.join(PROJECT_CONFIG_PATH)
    } else {
      PathBuf::from(PROJECT_CONFIG_PATH)
    };

    let exists = exists(&conf_path).wrap_err("Failed to check if project-config exists")?;

    if !exists {
      return Err(eyre!("Config not found in current directory!"));
    }

    let config = fs::read_to_string(&conf_path).wrap_err("Failed to read config")?;

    let toml_config: ProjectConfig =
      toml::from_str(&config).wrap_err("Failed to prase Project config")?;

    Ok(toml_config)
  }
}
