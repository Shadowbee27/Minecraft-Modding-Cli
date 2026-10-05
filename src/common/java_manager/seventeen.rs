use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::Java;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Java17 {
  pub version: String,
  pub vendor: String,
  pub arch: String,
  pub path: PathBuf,
}

impl Java for Java17 {
  fn install(self) -> color_eyre::eyre::Result<()> {
    unimplemented!()
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}
