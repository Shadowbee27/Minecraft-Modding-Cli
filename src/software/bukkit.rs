use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

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
  fn install(&self, pconf: ProjectConfig, pdir: &Path) {
    unimplemented!()
  }

  fn update(&self) {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Vec<SupportedVersion> {
    unimplemented!()
  }
}
