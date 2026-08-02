use super::SeverSoftware;

const PAPER_URL: &str = "";

pub struct Paper {}

impl SeverSoftware for Paper {
  fn install(&self) -> Result<(), super::SoftwareInstallError> {
    unimplemented!()
  }

  fn update(&self) -> Result<(), super::SoftwareUpdateError> {
    unimplemented!()
  }
}
