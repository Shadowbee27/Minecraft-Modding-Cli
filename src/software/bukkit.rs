use color_eyre::eyre::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bukkit {
  pub flavor_version: Version,
}

impl Default for Bukkit {
  fn default() -> Self {
    Bukkit {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Bukkit {
  fn install(&mut self, _pconf: ProjectConfig, _pdir: &Path) -> Result<()> {
    unimplemented!()
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    unimplemented!()
  }
}
