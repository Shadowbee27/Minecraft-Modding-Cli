use std::path::PathBuf;

use super::Java;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Java17 {
  pub version: String,
  pub vendor: String,
  pub arch: String,
  pub path: PathBuf,
}

impl Java for Java17 {
  fn install() -> color_eyre::eyre::Result<()> {
    unimplemented!()
  }
}
