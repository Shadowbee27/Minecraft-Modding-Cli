use color_eyre::Result;
use color_eyre::eyre::Context;
use rayon::prelude::*;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;
use tokio::runtime::Runtime;

use super::ServerSoftware;
use super::Software;
use super::SupportedVersion;
use crate::software::McVersion;
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::flavor;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::WhatToUpdate;
use crate::tui::dialogs::update::what_to_update;
use crate::{common::reqwest::download::download_file, config::project_conf::ProjectConfig};

const BABRIC_URL: &str = "https://meta.babric.glass-launcher.net/v2/versions";
const BABRIC_DOWNLOAD_URL: &str = "/loader/";

#[derive(Debug, Clone, Deserialize)]
pub struct BabricInfo {
  #[serde(rename(deserialize = "installer"))]
  pub installer_version: Vec<BabricInstallerVersion>,
  #[serde(rename(deserialize = "loader"))]
  pub loader_version: Vec<BabricLoaderVersion>,
  #[serde(rename(deserialize = "game"))]
  pub game_version: Vec<BabricGameVersions>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BabricInstallerVersion {
  pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BabricLoaderVersion {
  pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BabricGameVersions {
  pub version: String,
  pub stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Babric {
  pub flavor_version: Version,
  pub installer_version: Version,
}

impl BabricGameVersions {
  pub fn to_supported_version(&self) -> SupportedVersion {
    SupportedVersion::from_string_bool(self.version.clone(), self.stable)
  }
}

impl Default for Babric {
  fn default() -> Self {
    Babric {
      flavor_version: Version::new(0, 19, 3),
      installer_version: Version::new(1, 1, 2),
    }
  }
}

impl Software for Babric {
  const NAME: &str = "Barbric";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    let client = AClient::new();
    self.get_self_versions(&pconf.mc_version)?;

    // BSP: https://meta.Babricmc.net/v2/versions/loader/26.2/0.19.3/0.8.0/server/jar
    let download_url = format!(
      "{}{}{}/{}/{}/server/jar",
      BABRIC_URL,
      BABRIC_DOWNLOAD_URL,
      pconf.mc_version.0,
      self.flavor_version,
      self.installer_version
    );

    let file_path = format!(
      "{}/Babric-server-mc{}-Babric{}.jar",
      pdir.display(),
      pconf.mc_version.0,
      self.flavor_version
    );

    let rt = Runtime::new().unwrap();

    let result = rt.block_on(download_file(&client, &download_url, &file_path));

    pconf.flavor = super::ServerSoftware::BABRIC(self.clone());
    pconf.jar_file = PathBuf::from(file_path);

    pconf.save(&Some(pdir.to_path_buf()))?;

    result
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let babric = ServerSoftware::BABRIC(Babric::default());
    match what_to_update(&babric)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&babric)?;
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
    let info = get_babric_info()?;
    Ok(
      info
        .game_version
        .par_iter()
        .map(|v| v.to_supported_version())
        .collect(),
    )
  }

  fn get_self_versions(&mut self, _mc_version: &McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    let info = get_babric_info().wrap_err_with(|| "Error getting Metadata: {}")?;

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

fn get_babric_info() -> Result<BabricInfo> {
  let client = BClient::new();

  let response = client.get(BABRIC_URL).send()?.json::<BabricInfo>()?;

  Ok(response)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_babric_info_test() {
    dbg!(get_babric_info().unwrap());
  }

  #[test]
  fn get_forge_mc_versions() {
    dbg!(Babric::get_supported_minecraft_versions().unwrap());
  }
}
