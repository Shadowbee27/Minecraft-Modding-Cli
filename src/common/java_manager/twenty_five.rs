use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::Java;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Java25 {
  pub version: String,
  pub vendor: String,
  pub arch: String,
  pub path: PathBuf,
}

impl Java for Java25 {
  fn install(self) -> Result<()> {
    unimplemented!()
  }

  fn get_path(&self) -> PathBuf {
    self.path.clone()
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}
