use crate::config::ProjectConfig;

use super::McVersion;
use super::SeverSoftware;
use semver::Version;
use serde::{Deserialize, Serialize};

const FOLIA_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folia {
  pub flavor_version: Version,
}

impl SeverSoftware for Folia {
  fn install(&self, pconf: ProjectConfig) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
