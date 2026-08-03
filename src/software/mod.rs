use semver::Version;
use serde::{Deserialize, Serialize};

pub mod fabric;
pub mod folia;
pub mod forge;
pub mod neoforge;
pub mod paper;
pub mod vanilla;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ServerSoftware {
  FABRIC,
  FOLIA,
  FORGE,
  NEOFORGE,
  PAPER,
  VANILLA,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McVersion(Version);

pub enum SoftwareUpdateError {}
pub enum SoftwareInstallError {}

pub trait SeverSoftware {
  fn update(&self) -> Result<(), SoftwareUpdateError>;
  fn install(&self) -> Result<(), SoftwareInstallError>;
}
