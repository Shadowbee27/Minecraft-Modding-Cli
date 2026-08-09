use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;

const PURPUR_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purpur {
  pub flavor_version: Version,
}

impl Default for Purpur {
  fn default() -> Self {
    Purpur {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Purpur {
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
