use super::SeverSoftware;

const VANILLA_URL: &str = "";

pub struct Vanilla {}

impl SeverSoftware for Vanilla {
  fn install(&self) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
