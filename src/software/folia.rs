use color_eyre::Result;
use colorful::{Color, Colorful};
use rayon::prelude::*;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, fmt, path::Path, process::exit};
use tokio::runtime::Runtime;

use super::{Software, SupportedVersion};
use crate::common::reqwest::download::download_file;
use crate::config::project_conf::ProjectConfig;
use crate::software::{McVersion, ServerSoftware};
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::flavor::select_flavor_version;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::{WhatToUpdate, what_to_update};

const FOLIA_URL: &str = "https://fill.papermc.io/v3/projects/folia";

#[derive(Deserialize, Debug, Clone)]
pub struct FoliaMCVersions {
  pub versions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Folia {
  pub flavor_version: i32,
}

// https://fill.papermc.io/swagger-ui/index.html

impl Software for Folia {
  const NAME: &str = "Folia";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;
    let download_url = get_build_url(self.flavor_version, &pconf.mc_version.0)?;
    let file_path = format!(
      "{}/folia-server-mc{}-build{}.jar",
      pdir.display(),
      pconf.mc_version.0,
      self.flavor_version
    );

    let client = AClient::new();
    let rt = Runtime::new().unwrap();

    let download_result = rt.block_on(download_file(&client, &download_url, &file_path));

    pconf.flavor = super::ServerSoftware::FOLIA(self.clone());
    pconf.flavor_version = self.flavor_version.to_string();
    pconf.jar_file = PathBuf::from(file_path);

    pconf.save(&Some(pdir.to_path_buf()))?;

    download_result
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let folia = ServerSoftware::FOLIA(Folia::default());
    match what_to_update(&folia)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&folia)?;
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
    let versions = get_folia_metadata()?;

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

    Ok(versions_vec)
  }

  fn get_self_versions(&mut self, mc_version: &McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    let builds = match get_builds(&mc_version.0) {
      Ok(b) => b,
      Err(e) => {
        eprintln!(
          "Error getting build metadata: {}",
          e.to_string().color(Color::Red).bold()
        );
        exit(1)
      }
    };
    self.flavor_version = select_flavor_version(builds)?.parse::<i32>()?;

    Ok(())
  }
}

fn get_folia_metadata() -> color_eyre::Result<FoliaMCVersions> {
  let client = BClient::new();
  let info = client.get(FOLIA_URL).send()?.json::<FoliaMCVersions>()?;
  Ok(info)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct FoliaBuildInfo {
  downloads: FoliaBuildMeta,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct FoliaBuildMeta {
  #[serde(rename(deserialize = "server:default"))]
  server: FoliaBuildDownloadMeta,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct FoliaBuildDownloadMeta {
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
      FOLIA_URL, mc_version, build
    ))
    .send()?
    .json::<FoliaBuildInfo>()?;
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
    .get(format!("{}/versions/{}/builds", FOLIA_URL, mc_version))
    .send()?
    .json::<BuildList>()?;
  Ok(info.par_iter().map(|b| b.id.to_string()).collect())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::debug;

  #[test]
  fn folia_metadata() {
    debug!(get_folia_metadata()).unwrap();
  }

  #[test]
  fn folia_supported_version() {
    debug!(Folia::get_supported_minecraft_versions().unwrap());
  }

  #[test]
  fn test_get_builds() {
    debug!(get_builds("26.2").unwrap());
  }
}
