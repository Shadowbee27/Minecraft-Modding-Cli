use crate::software::ServerSoftware;
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::flavor::select_flavor_version;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::WhatToUpdate;
use crate::tui::dialogs::update::what_to_update;
use color_eyre::Result;
use rayon::prelude::*;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use tokio::runtime::Runtime;

use super::Software;
use super::SupportedVersion;
use crate::common::reqwest::download::download_file;
use crate::config::project_conf::ProjectConfig;

const PAPER_URL: &str = "https://fill.papermc.io/v3/projects/paper";

#[derive(Deserialize, Debug, Clone)]
pub struct PaperMCVersions {
  pub versions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paper {
  pub flavor_version: i32,
}

// https://fill.papermc.io/swagger-ui/index.html

impl Software for Paper {
  const NAME: &str = "Paper";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;

    let download_url = get_build_url(self.flavor_version, &pconf.mc_version.0)?;

    let file_path = format!(
      "{}/paper-server-mc{}-build{}.jar",
      pdir.display(),
      pconf.mc_version.0,
      self.flavor_version
    );

    let client = AClient::new();
    let rt = Runtime::new().unwrap();

    let download_result = rt.block_on(download_file(&client, &download_url, &file_path));

    pconf.flavor = super::ServerSoftware::PAPER(self.clone());
    pconf.flavor_version = self.flavor_version.to_string();
    pconf.jar_file = PathBuf::from(file_path);

    pconf.save(&Some(pdir.to_path_buf()))?;

    download_result
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let paper = ServerSoftware::PAPER(Paper::default());
    match what_to_update(&paper)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&paper)?;
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
    let versions = get_paper_metadata()?;
    let mut versions_vec = Vec::new();

    for i in versions.versions {
      for v in i.1 {
        if v.contains('-') {
          versions_vec.push(SupportedVersion::from_string_bool(v, false));
        } else {
          versions_vec.push(SupportedVersion::from_string_bool(v, true));
        }
      }
    }

    versions_vec.sort();
    versions_vec.reverse();
    Ok(versions_vec)
  }

  fn get_self_versions(&mut self, mc_version: &super::McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    let mc_version: &str = &mc_version.0;
    let client = BClient::new();
    let info = client
      .get(format!("{}/versions/{}/builds", PAPER_URL, mc_version))
      .send()?
      .json::<BuildList>()?;
    let builds = info.par_iter().map(|b| b.id.to_string()).collect();

    self.flavor_version = select_flavor_version(builds)?.parse::<i32>()?;

    Ok(())
  }
}

fn get_paper_metadata() -> Result<PaperMCVersions> {
  let client = BClient::new();
  let info = client.get(PAPER_URL).send()?.json::<PaperMCVersions>()?;
  Ok(info)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct PaperBuildInfo {
  downloads: PaperBuildMeta,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct PaperBuildMeta {
  #[serde(rename(deserialize = "server:default"))]
  server: PaperBuildDownloadMeta,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct PaperBuildDownloadMeta {
  url: String,
}

fn get_build_url<T: fmt::Display, G: fmt::Display>(
  build: T,
  mc_version: G,
) -> color_eyre::Result<String> {
  let client = BClient::new();
  let info = client
    .get(format!(
      "{}/versions/{}/builds/{}",
      PAPER_URL, mc_version, build
    ))
    .send()?
    .json::<PaperBuildInfo>()?;
  Ok(info.downloads.server.url)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
  pub id: u64,
}

pub type BuildList = Vec<Build>;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::debug;

  #[test]
  fn paper_metadata() {
    debug!(get_paper_metadata()).unwrap();
  }

  #[test]
  fn paper_supported_version() {
    debug!(Paper::get_supported_minecraft_versions().unwrap());
  }
}
