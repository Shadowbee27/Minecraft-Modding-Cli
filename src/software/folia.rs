use super::SeverSoftware;

const FOLIA_URL: &str = "";

pub struct Folia {}

impl SeverSoftware for Folia {
  fn install(&self) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
