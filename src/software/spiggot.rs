use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;

const SPIGGOT_URL: &str = "";

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
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path) {
    unimplemented!()
  }

  fn update(&mut self) {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Vec<SupportedVersion> {
    unimplemented!()
  }
}
