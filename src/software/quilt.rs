use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quilt {
  pub flavor_version: Version,
}

impl Default for Quilt {
  fn default() -> Self {
    Quilt {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Quilt {
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
