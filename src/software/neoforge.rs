use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::runtime::Runtime;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::common::maven::get_versions as get_maven_versions;
use crate::config::ProjectConfig;

const NEOFORGE_URL: &str = "https://maven.neoforged.net/releases";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeoForge {
  pub flavor_version: String,
}

impl SeverSoftware for NeoForge {
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path) {
    unimplemented!()
  }

  fn update(&mut self) {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Vec<SupportedVersion> {
    unimplemented!()
  }
}

fn get_versions() -> anyhow::Result<Vec<String>> {
  let rt = Runtime::new().unwrap();
  let meta = rt.block_on(get_maven_versions("net/neoforged/neoforge", NEOFORGE_URL))?;
  Ok(meta.versioning.versions.list)
}
