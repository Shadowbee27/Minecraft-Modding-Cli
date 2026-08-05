use super::SeverSoftware;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

const NEOFORGE_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoForge {
  pub flavor_version: Version,
}

impl SeverSoftware for NeoForge {
  fn install(&self, pconf: ProjectConfig, pdir: &Path) {
    unimplemented!()
  }

  fn update(&self) {
    unimplemented!()
  }
}
