use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use super::software::{McVersion, ServerSoftware};

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
  pub flavor_version: String,
  pub mc_version: McVersion,
  pub jar_file: String,
}

impl ProjectConfig {
  pub fn save(&self, project_dir: &Path) -> anyhow::Result<()> {
    let serialized = toml::to_string(&self)?;

    fs::write(project_dir.join(CONFIG_PATH), serialized)?;
    Ok(())
  }
}
