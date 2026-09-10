use crate::tui::dialogs::flavor::select_flavor_version;
use color_eyre::Result;
use rayon::prelude::*;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use tokio::runtime::Runtime;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::common::download::download_file;
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

impl SeverSoftware for Paper {
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;

    let download_url = get_build_url(self.flavor_version, &pconf.mc_version.0)?;

    let file_path = format!(
      "paper-server-mc{}-build{}.jar",
      pconf.mc_version.0, self.flavor_version
    );

    let client = AClient::new();
    let rt = Runtime::new().unwrap();

    let download_result = rt.block_on(download_file(
      &client,
      &download_url,
      &format!("{}/{}", pdir.to_str().unwrap(), file_path),
    ));

    pconf.flavor = super::ServerSoftware::PAPER(self.clone());
    pconf.flavor_version = self.flavor_version.to_string();
    pconf.jar_file = file_path;

    pconf.save(pdir)?;

    download_result
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    let versions = get_paper_metadata()?;
    let mut versions_vec = Vec::new();

    for i in versions.versions {
      for v in i.1 {
        if v.contains('-') {
          versions_vec.push(SupportedVersion {
            version: v,
            stable: false,
          });
        } else {
          versions_vec.push(SupportedVersion {
            version: v,
            stable: true,
          });
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
    let builds = get_builds(&mc_version.0)?;

    self.flavor_version = select_flavor_version(builds)?.parse::<i32>()?;

    Ok(())
  }
}

fn get_paper_metadata() -> color_eyre::Result<PaperMCVersions> {
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

fn get_builds(mc_version: &str) -> color_eyre::Result<Vec<String>> {
  let client = BClient::new();
  let info = client
    .get(format!("{}/versions/{}/builds", PAPER_URL, mc_version))
    .send()?
    .json::<BuildList>()?;
  Ok(info.par_iter().map(|b| b.id.to_string()).collect())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn paper_metadata() {
    dbg!(get_paper_metadata()).unwrap();
  }

  #[test]
  fn paper_supported_version() {
    dbg!(Paper::get_supported_minecraft_versions().unwrap());
  }
}
