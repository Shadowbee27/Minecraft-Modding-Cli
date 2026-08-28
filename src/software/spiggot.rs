use color_eyre::eyre::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;

const _SPIGGOT_URL: &str = "hub.spigotmc.org";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spiggot {
  pub flavor_version: Version,
}

impl Default for Spiggot {
  fn default() -> Self {
    Spiggot {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Spiggot {
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
