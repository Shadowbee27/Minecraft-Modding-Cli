use color_eyre::{
  Result,
  eyre::{Context, eyre},
};
use semver::{BuildMetadata, Prerelease, Version};
use std::fs::{self, exists};

use crate::config::ProjectConfig;

pub const PROJECT_CONFIG_PATH: &str = ".mcmc/config.toml";
pub const PROJECT_CONFIG_VERSION: Version = Version {
  major: 0,
  minor: 0,
  patch: 0,
  pre: Prerelease::EMPTY,
  build: BuildMetadata::EMPTY,
};

pub fn load_project_config() -> Result<ProjectConfig> {
  let exists = exists(PROJECT_CONFIG_PATH).wrap_err("Failed to check if project-config exists")?;

  if !exists {
    return Err(eyre!("Config not found in current directory!"));
  }

  let config = fs::read_to_string(PROJECT_CONFIG_PATH).wrap_err("Failed to read config")?;

  let toml_config: ProjectConfig =
    toml::from_str(&config).wrap_err("Failed to prase Project config")?;

  Ok(toml_config)
}
