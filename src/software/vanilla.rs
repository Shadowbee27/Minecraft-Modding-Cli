use super::SeverSoftware;
use crate::config::ProjectConfig;
use semver::Version;
use serde::{Deserialize, Serialize};

const VANILLA_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vanilla {
  pub flavor_version: Version,
}

impl SeverSoftware for Vanilla {
  fn install(&self, pconf: ProjectConfig) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
