use std::{env, path::PathBuf};

pub fn get_buildtools_build_dir() -> PathBuf {
  env::temp_dir()
}
