use colorful::{Color, Colorful};
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::exit;
use tokio::runtime::Runtime;

use super::{SeverSoftware, SupportedVersion};
use crate::common::download::download_file;
use crate::config::ProjectConfig;
use crate::tui::dialogs::flavor::select_flavor_version;

const PURPUR_URL: &str = "https://api.purpurmc.org/v2/purpur";

#[derive(Debug, Clone, Deserialize)]
pub struct PurpurMCInfo {
  pub versions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PurpurBuilds {
  pub latest: String,
  pub all: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PurpurVersionInfo {
  pub builds: PurpurBuilds,
}

impl PurpurVersionInfo {
  fn to_vec(&self) -> Vec<String> {
    let mut result = Vec::new();
    result.push(self.builds.latest.clone());
    result.append(&mut self.builds.all.clone());
    result
  }
}

impl PurpurMCInfo {
  pub fn to_supported_version(&self) -> Vec<SupportedVersion> {
    let mut supported_version = Vec::new();
    for v in self.versions.clone() {
      supported_version.push(SupportedVersion {
        version: v,
        stable: true,
      });
    }
    supported_version.reverse(); // needed as versions are provided older -> newer
    supported_version
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Purpur {
  pub flavor_version: String,
}

impl SeverSoftware for Purpur {
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) {
    let info = match get_purpur_versions(&pconf.mc_version.0) {
      Ok(m) => m,
      Err(e) => {
        eprintln!(
          "Error getting Metadata: {}",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    };
    self.flavor_version =
      select_flavor_version(info.to_vec().iter().map(|v| v.as_str()).collect()).to_string();

    let download_url = format!(
      "{}/{}/{}/download",
      PURPUR_URL, pconf.mc_version.0, self.flavor_version
    );

    let file_path = format!(
      "purpur-server-mc{}-{}.jar",
      pconf.mc_version.0, self.flavor_version
    );

    let client = AClient::new();
    let rt = Runtime::new().unwrap();

    let download_result = rt.block_on(download_file(
      &client,
      &download_url,
      &format!("{}/{}", pdir.to_str().unwrap(), file_path),
    ));

    pconf.flavor = super::ServerSoftware::PURPUR(self.clone());
    pconf.jar_file = file_path;

    match pconf.save(pdir) {
      Ok(_) => {}
      Err(e) => {
        eprintln!(
          "Error while saving config file: ({})",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    }

    match download_result {
      Ok(_) => {}
      Err(e) => {
        eprintln!(
          "Error while downloading file: ({})",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    };
  }

  fn update(&mut self) {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Vec<SupportedVersion> {
    let info = match get_purpur_mc_versions() {
      Ok(meta) => meta,
      Err(e) => {
        eprintln!(
          "Error getting Metadata: {}",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    };
    info.to_supported_version()
  }
}

fn get_purpur_mc_versions() -> anyhow::Result<PurpurMCInfo> {
  let c = BClient::new();
  let info = c.get(PURPUR_URL).send()?.json::<PurpurMCInfo>()?;
  Ok(info)
}

fn get_purpur_versions(mc_version: &str) -> anyhow::Result<PurpurVersionInfo> {
  let c = BClient::new();

  let info = c
    .get(format!("{}/{}", PURPUR_URL, mc_version))
    .send()?
    .json::<PurpurVersionInfo>()?;
  Ok(info)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn purpur_mc_version_meta() {
    let _ = dbg!(get_purpur_mc_versions()).unwrap();
  }

  #[test]
  fn purpur_version_meta() {
    dbg!(get_purpur_versions("26.2")).unwrap();
  }
}
