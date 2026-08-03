use super::SeverSoftware;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};

const FORGE_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forge {
  pub flavor_version: Version,
}

impl SeverSoftware for Forge {
  fn install(&self, pconf: ProjectConfig) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
