use super::software::{McVersion, ServerSoftware};
use semver::Version;
use serde::{Deserialize, Serialize};

pub const CONFIG_PATH: &str = ".mcmc/config.toml";

#[derive(Deserialize, Serialize)]
pub struct ProjectConfig {
  pub config_version: Version,
  pub software: ServerSoftware,
  pub mc_version: McVersion,
}
