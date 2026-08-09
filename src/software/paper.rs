use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;

const PAPER_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
  pub flavor_version: Version,
}

// https://fill.papermc.io/swagger-ui/index.html

impl Default for Paper {
  fn default() -> Self {
    Paper {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Paper {
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
