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
use crate::software::McVersion;
use crate::tui::dialogs::flavor;
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

  fn update(&mut self) -> Result<()> {
    unimplemented!()
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
    let info = match get_babric_info() {
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

fn get_babric_info() -> color_eyre::Result<BabricInfo> {
  let client = BClient::new();

  let response = client.get(BABRIC_URL).send()?.json::<BabricInfo>()?;

  Ok(response)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::debug;

  #[test]
  fn babric_info_test() {
    let result = get_babric_info();
    debug!(result).unwrap();
  }
}
