use super::SeverSoftware;

const FABRIC_URL: &str = "";

pub struct Fabric {}

impl SeverSoftware for Fabric {
  fn install(&self) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
