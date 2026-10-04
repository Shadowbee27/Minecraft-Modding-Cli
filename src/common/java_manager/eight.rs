use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Java;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Java8 {
  pub version: String,
  pub vendor: String,
  pub arch: String,
  pub path: PathBuf,
}

impl Java for Java8 {
  fn install(self) -> color_eyre::eyre::Result<()> {
    unimplemented!()
  }
}
