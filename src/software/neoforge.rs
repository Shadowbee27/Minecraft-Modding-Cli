use super::SeverSoftware;

const NEOFORGE_URL: &str = "";

pub struct NeoForge {}

impl SeverSoftware for NeoForge {
  fn install(&self) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
