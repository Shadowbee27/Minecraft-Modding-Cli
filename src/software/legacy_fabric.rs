use color_eyre::Result;
use colorful::{Color, Colorful};
use rayon::prelude::*;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use tokio::runtime::Runtime;

use super::Software;
use super::SupportedVersion;
use crate::software::ServerSoftware;
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::flavor;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::WhatToUpdate;
use crate::tui::dialogs::update::what_to_update;
use crate::{common::reqwest::download::download_file, config::project_conf::ProjectConfig};

const LEGACY_FABRIC_URL: &str = "https://meta.legacyfabric.net/v2/versions";
const LEGACY_FABRIC_DOWNLOAD_URL: &str = "/loader/";

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyFabricInfo {
  #[serde(rename(deserialize = "installer"))]
  pub installer_version: Vec<LegacyFabricInstallerVersion>,
  #[serde(rename(deserialize = "loader"))]
  pub loader_version: Vec<LegacyFabricLoaderVersion>,
  #[serde(rename(deserialize = "game"))]
  pub game_version: Vec<LegacyFabricGameVersions>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyFabricInstallerVersion {
  pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyFabricLoaderVersion {
  pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyFabricGameVersions {
  pub version: String,
  pub stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyFabric {
  pub flavor_version: Version,
  pub installer_version: Version,
}

impl LegacyFabricGameVersions {
  pub fn to_supported_version(&self) -> SupportedVersion {
    SupportedVersion::from_string_bool(self.version.clone(), self.stable)
  }
}

impl Default for LegacyFabric {
  fn default() -> Self {
    LegacyFabric {
      flavor_version: Version::new(0, 19, 3),
      installer_version: Version::new(1, 1, 2),
    }
  }
}

impl Software for LegacyFabric {
  const NAME: &str = "Legacy Farbic";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;
    let client = AClient::new();
    // BSP: https://meta.fabricmc.net/v2/versions/loader/26.2/0.19.3/0.8.0/server/jar
    let download_url = format!(
      "{}{}{}/{}/{}/server/jar",
      LEGACY_FABRIC_URL,
      LEGACY_FABRIC_DOWNLOAD_URL,
      pconf.mc_version.0,
      self.flavor_version,
      self.installer_version
    );

    let file_path = format!(
      "{}/legacy-fabric-server-mc{}-fabric{}.jar",
      pdir.display(),
      pconf.mc_version.0,
      self.flavor_version
    );

    let rt = Runtime::new().unwrap();

    let result = rt.block_on(download_file(&client, &download_url, &file_path));

    pconf.flavor = super::ServerSoftware::LegacyFabric(self.clone());
    pconf.jar_file = PathBuf::from(file_path);

    pconf.save(&Some(pdir.to_path_buf()))?;

    result
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let legacy_fabric = ServerSoftware::LegacyFabric(LegacyFabric::default());
    match what_to_update(&legacy_fabric)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&legacy_fabric)?;
        if mc_ver < pconf.mc_version
          && !yes_no_prompt(
            "You are downgrading, this may corrupt your world are you sure?",
            None,
          )
        {
          return Ok(());
        }

        pconf.mc_version = mc_ver;
      }
      WhatToUpdate::FLAVOR(_) => {}
    };

    let pdir = if let Some(dir) = pdir {
      dir
    } else {
      PathBuf::from(".")
    };
    self.install(pconf, <&Path>::from(&pdir))?;
    Ok(())
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    let info = match get_fabric_info() {
      Ok(meta) => meta,
      Err(e) => {
        eprintln!(
          "Error getting Metadata: {}",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    };

    Ok(
      info
        .game_version
        .par_iter()
        .map(|v| v.to_supported_version())
        .collect(),
    )
  }

  fn get_self_versions(&mut self, _mc_version: &super::McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    let info = match get_fabric_info() {
      Ok(meta) => meta,
      Err(e) => {
        eprintln!(
          "Error getting Metadata: {}",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    };

    self.flavor_version = Version::parse(flavor::select_flavor_version(
      info
        .loader_version
        .par_iter()
        .map(|v| v.version.as_str())
        .collect(),
    )?)?;

    self.installer_version = Version::parse(flavor::select_fabric_installer_version(
      info
        .installer_version
        .par_iter()
        .map(|v| v.version.as_str())
        .collect(),
    )?)?;

    Ok(())
  }
}

fn get_fabric_info() -> color_eyre::Result<LegacyFabricInfo> {
  let client = BClient::new();

  let response = client
    .get(LEGACY_FABRIC_URL)
    .send()?
    .json::<LegacyFabricInfo>()?;

  Ok(response)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::debug;

  #[test]
  fn fabric_info_test() {
    let result = get_fabric_info();
    debug!(result).unwrap();
  }
}
