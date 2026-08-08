use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FORGE_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forge {
  pub flavor_version: Version,
}

impl Default for Forge {
  fn default() -> Self {
    Forge {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Forge {
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
