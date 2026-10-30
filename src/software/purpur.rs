use color_eyre::Result;
use rayon::prelude::*;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::runtime::Runtime;

use super::{Software, SupportedVersion};
use crate::common::reqwest::download::download_file;
use crate::config::project_conf::ProjectConfig;
use crate::software::ServerSoftware;
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::flavor::select_flavor_version;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::{WhatToUpdate, what_to_update};

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
      supported_version.push(SupportedVersion::from_string_bool(v, true));
    }
    supported_version.reverse(); // needed as versions are provided older -> newer
    supported_version
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Purpur {
  pub flavor_version: String,
}

impl Software for Purpur {
  const NAME: &str = "Purpur";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;

    let download_url = format!(
      "{}/{}/{}/download",
      PURPUR_URL, pconf.mc_version.0, self.flavor_version
    );

    let file_path = format!(
      "{}/purpur-server-mc{}-{}.jar",
      pdir.display(),
      pconf.mc_version.0,
      self.flavor_version
    );

    let client = AClient::new();
    let rt = Runtime::new().unwrap();

    let download_result = rt.block_on(download_file(&client, &download_url, &file_path));

    pconf.flavor = super::ServerSoftware::PURPUR(self.clone());
    pconf.flavor_version = self.flavor_version.clone();
    pconf.jar_file = PathBuf::from(file_path);

    pconf.save(&Some(pdir.to_path_buf()))?;

    download_result
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let purpur = ServerSoftware::PURPUR(Purpur::default());
    match what_to_update(&purpur)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&purpur)?;
        if mc_ver < pconf.mc_version
          && !yes_no_prompt(
            "You are downgrading, this may corrupt your world are you sure?",
            None,
          ) {
            return Ok(());
          }
        pconf.mc_version = mc_ver;
      }
      WhatToUpdate::FLAVOR(_) => {}
    };

    pconf.save(&pdir)?;

    let pdir = if let Some(dir) = pdir {
      dir
    } else {
      PathBuf::from(".")
    };
    self.install(pconf, <&Path>::from(&pdir))?;
    Ok(())
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    Ok(get_purpur_mc_versions()?.to_supported_version())
  }

  fn get_self_versions(&mut self, mc_version: &super::McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    let info = get_purpur_versions(&mc_version.0)?;
    self.flavor_version =
      select_flavor_version(info.to_vec().par_iter().map(|v| v.as_str()).collect())?.to_string();
    Ok(())
  }
}

fn get_purpur_mc_versions() -> color_eyre::Result<PurpurMCInfo> {
  let c = BClient::new();
  let info = c.get(PURPUR_URL).send()?.json::<PurpurMCInfo>()?;
  Ok(info)
}

fn get_purpur_versions(mc_version: &str) -> color_eyre::Result<PurpurVersionInfo> {
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
  use crate::debug;

  #[test]
  fn purpur_mc_version_meta() {
    let _ = debug!(get_purpur_mc_versions()).unwrap();
  }

  #[test]
  fn purpur_version_meta() {
    debug!(get_purpur_versions("26.2")).unwrap();
  }
}
