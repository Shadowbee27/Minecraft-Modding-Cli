use std::{fs, path::Path};

use super::software::{McVersion, ServerSoftware};
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};

pub const CONFIG_PATH: &str = ".mcmc/config.toml";
pub const CONFIG_VERSION: Version = Version {
  major: 0,
  minor: 0,
  patch: 0,
  pre: Prerelease::EMPTY,
  build: BuildMetadata::EMPTY,
};

#[derive(Deserialize, Serialize)]
pub struct ProjectConfig {
  pub config_version: Version,
  pub flavor: ServerSoftware,
  pub flavor_version: Version,
  pub mc_version: McVersion,
}

impl ProjectConfig {
  pub fn write_to_file(self, project_dir: &Path) -> anyhow::Result<ProjectConfig> {
    let serialized = toml::to_string(&self)?;

    fs::write(project_dir.join(CONFIG_PATH), serialized)?;
    Ok(self)
  }
}
