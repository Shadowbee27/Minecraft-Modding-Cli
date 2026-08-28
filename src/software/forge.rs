use color_eyre::eyre::Result;
use rayon::prelude::*;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::runtime::Runtime;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::common::maven::get_versions;
use crate::config::ProjectConfig;

const _FORGE_URL: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forge {
  pub flavor_version: Version,
}

impl Default for Forge {
  fn default() -> Self {
    Forge {
      flavor_version: Version::new(0, 0, 0),
    }
  }
}

impl SeverSoftware for Forge {
  fn install(&mut self, _pconf: ProjectConfig, _pdir: &Path) -> Result<()> {
    unimplemented!()
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    let rt = Runtime::new()?;

    let versions = rt.block_on(get_versions(
      "net/minecraftforge/forge",
      "https://maven.minecraftforge.net/releases",
    ))?;

    let mut mc_versions = versions
      .versioning
      .versions
      .list
      .par_iter()
      .map(|v| {
        let version = v.split_once("-").expect("Failed to split version").0;

        SupportedVersion {
          version: version.to_owned(),
          stable: true,
        }
      })
      .collect::<Vec<SupportedVersion>>();

    mc_versions.sort();
    mc_versions.dedup();

    Ok(mc_versions)
  }
}

fn get_forge_versions() -> Result<Vec<String>> {
  let rt = Runtime::new()?;

  let versions = rt.block_on(get_versions(
    "net/minecraftforge/forge",
    "https://maven.minecraftforge.net/releases",
  ))?;

  Ok(
    versions
      .versioning
      .versions
      .list
      .par_iter()
      .map(|v| {
        v.split_once("-")
          .expect("Failed to split version")
          .1
          .to_owned()
      })
      .collect::<Vec<String>>(),
  )
}
