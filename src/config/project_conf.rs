use color_eyre::{
  Result,
  eyre::{Context, eyre},
};
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use std::{
  fs::{self, exists},
  path::PathBuf,
};

use crate::{
  common::java_manager::JavaVersion,
  debug,
  software::{McVersion, ServerSoftware},
  tui::modals::run::Ram,
};

pub const PROJECT_CONFIG_PATH: &str = ".mcmc/config.toml";
pub const PROJECT_CONFIG_VERSION: Version = Version {
  major: 0,
  minor: 0,
  patch: 0,
  pre: Prerelease::EMPTY,
  build: BuildMetadata::EMPTY,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProjectConfig {
  pub server_name: String,
  pub config_version: Version,
  pub flavor: ServerSoftware,
  pub flavor_version: String,
  pub mc_version: McVersion,
  pub jar_file: PathBuf,
  pub ram: Ram,
  pub java_install: Option<JavaVersion>,
}

impl ProjectConfig {
  pub fn save(&self, project_dir: &Option<PathBuf>) -> Result<()> {
    let serialized = toml::to_string(&self)?;
    if !self.jar_file.is_empty() {
      self.jar_file.canonicalize().unwrap();
    }

    let conf_path = if let Some(prefix) = project_dir {
      prefix.join(PROJECT_CONFIG_PATH)
    } else {
      PathBuf::from(PROJECT_CONFIG_PATH)
    };

    debug!(&conf_path);

    fs::write(conf_path, serialized)?;
    Ok(())
  }

  pub fn load_project_config(path_prefix: &Option<PathBuf>) -> Result<ProjectConfig> {
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
