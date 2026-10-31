use color_eyre::{
  Result,
  eyre::{ContextCompat, OptionExt, eyre},
};
use colorful::{Color, Colorful};
use rayon::prelude::*;
use reqwest::blocking::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::runtime::Runtime;

use super::Software;
use super::SupportedVersion;
use crate::{
  common::reqwest::download::download_file,
  software::ServerSoftware,
  tui::{
    common::simple_prompt::yes_no_prompt,
    dialogs::{minecraft::mc_version_select, update::what_to_update},
  },
};
use crate::{config::project_conf::ProjectConfig, tui::dialogs::update::WhatToUpdate};

const QUILT_URL: &str = "https://meta.quiltmc.org/v3/versions";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Quilt {}

#[derive(Debug, Clone, Deserialize)]
struct QuiltMeta {
  game: Vec<QuiltMetaGame>,
  // mappings: ,
  // hashed: ,
  // loader: ,
  installer: Vec<QuiltMetaInstaller>,
}

impl QuiltMetaGame {
  fn to_supported_versions(&self) -> SupportedVersion {
    SupportedVersion::from_string_bool(self.version.clone(), self.stable)
  }
}

#[derive(Debug, Clone, Deserialize)]
struct QuiltMetaGame {
  version: String,
  stable: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct QuiltMetaInstaller {
  version: String,
}

trait QuiltInstallerGetLatest {
  fn get_latest(&self) -> Version;
}

impl QuiltInstallerGetLatest for Vec<QuiltMetaInstaller> {
  fn get_latest(&self) -> Version {
    let mut versions = Vec::new();
    for q in self {
      versions.push(Version::parse(q.version.as_str()).expect("Failed to parse semver"));
    }
    versions
      .par_iter()
      .max()
      .expect("Failed to get latest installer version")
      .clone()
  }
}

impl Software for Quilt {
  const NAME: &str = "Quilt";
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    let meta = get_quilt_meta()?;

    let installer_version = meta.installer.get_latest().to_string();

    Runtime::new()
      .unwrap()
      .block_on(download_file(
        &reqwest::Client::new(),
        &format!("https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-installer/{}/quilt-installer-{}.jar", installer_version, installer_version),
        &format!("{}/quilt_installer.jar", pdir.to_str().wrap_err("Failed to convert project dir to string")?).to_string(),
      ))
    ?;
    // java -jar quilt-installer-INSTALLER_VERSION.jar install server MINECRAFT_VERSION --download-server
    let cmd = Command::new("java")
      .args([
        "-jar",
        format!(
          "{}/quilt_installer.jar",
          pdir
            .to_str()
            .ok_or_eyre("Failed to convert project dir to string")?
        )
        .as_str(),
        "install",
        "server",
        pconf.mc_version.0.as_str(),
        "--download-server",
        format!("--install-dir={}", pdir.display()).as_str(),
      ])
      .spawn()?;

    println!("----- Quilt-Installer output -----");
    let result = cmd.wait_with_output()?;

    let eoo = "----End of Quilt-Installer output----";
    if !result.status.success() {
      println!("{}", eoo.color(Color::Red).bold());
      return Err(eyre!("Failed to run Quilt-Installer, {}", result.status));
    }

    println!("{}", eoo.color(Color::Green).bold());
    Ok(())
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let quilt = ServerSoftware::QUILT(Quilt::default());
    match what_to_update(&quilt)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&quilt)?;
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
    let meta = get_quilt_meta()?;
    Ok(
      meta
        .game
        .par_iter()
        .map(|v| v.to_supported_versions())
        .collect(),
    )
  }
}

fn get_quilt_meta() -> Result<QuiltMeta> {
  let client = Client::new();
  Ok(client.get(QUILT_URL).send()?.json::<QuiltMeta>()?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn quilt_meta() {
    dbg!(get_quilt_meta().unwrap());
  }

  #[test]
  fn quilt_mc_versions() {
    dbg!(Quilt::get_supported_minecraft_versions().unwrap());
  }
}
