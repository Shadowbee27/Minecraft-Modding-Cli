use std::{env, path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Config {
  pub java_settings: JavaConfig,
  pub directories: DirsConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JavaConfig {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DirsConfig {}

pub fn get_buildtools_build_dir() -> PathBuf {
  env::temp_dir()
}
