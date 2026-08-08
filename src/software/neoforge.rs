use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

const NEOFORGE_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoForge {
  pub flavor_version: Version,
}

impl Default for NeoForge {
  fn default() -> Self {
    NeoForge {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for NeoForge {
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
