use super::SeverSoftware;
use crate::config::ProjectConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quilt {}

impl SeverSoftware for Quilt {
  fn install(&self, pconf: ProjectConfig, pdir: &Path) {
    unimplemented!()
  }

  fn update(&self) {
    unimplemented!()
  }
}
