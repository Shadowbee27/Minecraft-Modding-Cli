use color_eyre::{Result, eyre::Context};
use dirs::{config_dir, config_local_dir};
use serde::{Deserialize, Serialize};
use std::{
  env::temp_dir,
  fs::{File, create_dir_all, exists, read_to_string, write},
  io::Write,
  path::PathBuf,
};

use crate::common::java_manager::{
  eight::Java8, seventeen::Java17, twenty_five::Java25, twenty_one::Java21,
};

const CONFIG_PATH: &str = "mcmc/mcmc-config.toml";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Config {
  pub java_settings: JavaConfig,
  pub directories: DirsConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct JavaConfig {
  pub default_8: Java8,
  pub default_17: Java17,
  pub default_21: Java21,
  pub default_25: Java25,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DirsConfig {}

pub fn get_buildtools_build_dir() -> PathBuf {
  temp_dir()
}

impl Default for Config {
  fn default() -> Self {
    Config {
      java_settings: JavaConfig {
        default_8: Java8::default(),
        default_17: Java17::default(),
        default_21: Java21::default(),
        default_25: Java25::default(),
      },
      directories: DirsConfig {},
    }
  }
}

impl Config {
  pub fn save(&self) -> Result<()> {
    let serialized = toml::to_string_pretty(&self)?;

    let config_path = config_dir()
      .expect("Failed to get config dir")
      .join(CONFIG_PATH);

    if exists(&config_path)? {
      write(config_path, serialized)?;
    } else {
      if !exists(config_dir().expect("Failed to get config dir").join("mcmc"))? {
        create_dir_all(config_dir().expect("Failed to get config dir").join("mcmc"))?;
      }

      File::create(config_path)?.write_all(serialized.as_bytes())?;
    }

    Ok(())
  }

  pub fn load() -> Result<Self> {
    let config_path = config_local_dir()
      .expect("Failed to get config dir")
      .join(CONFIG_PATH);

    let exists = exists(&config_path).wrap_err("Failed to check if project-config exists")?;

    let toml_config: Config;

    if !exists {
      toml_config = Config::default();
      toml_config.save()?;
    } else {
      let config = read_to_string(&config_path).wrap_err("Failed to read config")?;

      toml_config = toml::from_str(&config).wrap_err("Failed to prase Project config")?;
    }

    Ok(toml_config)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_name() {
    dbg!(
      config_local_dir()
        .expect("Failed to get config dir")
        .join(CONFIG_PATH)
    );
    dbg!(Config::load().unwrap());
  }
}
