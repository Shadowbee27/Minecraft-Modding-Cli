use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FOLIA_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folia {
  pub flavor_version: Version,
}

impl Default for Folia {
  fn default() -> Self {
    Folia {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Folia {
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
