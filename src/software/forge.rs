use super::SeverSoftware;

const FORGE_URL: &str = "";

pub struct Forge {}

impl SeverSoftware for Forge {
  fn install(&self) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
