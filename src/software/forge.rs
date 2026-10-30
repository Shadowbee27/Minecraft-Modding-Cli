use color_eyre::eyre::eyre;
use color_eyre::{Result, eyre::OptionExt};
use colorful::{Color, Colorful};
use rayon::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::runtime::Runtime;

use super::Software;
use super::SupportedVersion;
use crate::common::maven::get_versions;
use crate::common::reqwest::download::download_file;
use crate::config::project_conf::ProjectConfig;
use crate::software::{McVersion, ServerSoftware};
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::flavor::select_flavor_version;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::{WhatToUpdate, what_to_update};

const FORGE_URL: &str = "https://maven.minecraftforge.net/releases";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Forge {
  pub flavor_version: String,
}

impl Software for Forge {
  const NAME: &str = "Forge";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;

    self.flavor_version = format!("{}-{}", pconf.mc_version.clone().0, self.flavor_version);

    // https://maven.minecraftforge.net/net/minecraftforge/forge/26.2-65.1.3/forge-26.2-65.1.3-installer.jar
    let url = format!(
      "https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar",
      self.flavor_version, self.flavor_version
    );
    pconf.jar_file = PathBuf::from(format!(
      "{}/forge-{}-installer.jar",
      pdir.display(),
      self.flavor_version
    ));

    Runtime::new()?.block_on(download_file(
      &Client::new(),
      url.as_str(),
      &pconf.jar_file.display().to_string(),
    ))?;

    // java -jar test_server/forge-26.2-65.1.3-installer.jar --installServer test_server
    let cmd = Command::new("java")
      .args([
        "-jar",
        &pconf.jar_file.display().to_string(),
        "--installServer",
        pdir
          .to_str()
          .ok_or_eyre("Failed to convert path to string")?,
      ])
      .spawn()?;

    pconf.flavor = ServerSoftware::FORGE(self.clone());
    pconf.flavor_version = self.flavor_version.clone();
    pconf.save(&Some(pdir.to_path_buf()))?;

    println!("----- Forge-Installer output -----");
    let result = cmd.wait_with_output()?;

    let eoo = "----End of Forge-Installer output----";

    if !result.status.success() {
      println!("{}", eoo.color(Color::Red).bold());
      return Err(eyre!("Failed to run Forge-Installer, {}", result.status));
    }

    println!("{}", eoo.color(Color::Red).bold());
    Ok(())
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let forge = ServerSoftware::FORGE(Forge::default());
    match what_to_update(&forge)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&forge)?;
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

        SupportedVersion::from_string_bool(version.to_owned(), true)
      })
      .collect::<Vec<SupportedVersion>>();

    mc_versions.sort();
    mc_versions.dedup();
    mc_versions.reverse();

    Ok(mc_versions)
  }

  fn get_self_versions(&mut self, mc_version: &McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    self.flavor_version = select_flavor_version(get_forge_versions_for_mc_version(mc_version)?)?;
    Ok(())
  }
}

fn get_forge_versions_for_mc_version(mc_version: &McVersion) -> Result<Vec<String>> {
  let rt = Runtime::new()?;

  let versions = rt.block_on(get_versions("net/minecraftforge/forge", FORGE_URL))?;

  let split_versions = versions
    .versioning
    .versions
    .list
    .par_iter()
    .map(|v| v.split_once("-").expect("Failed to split version"))
    .collect::<Vec<(&str, &str)>>();

  let mut mc_forge_version = Vec::new();

  for (mc, forge) in split_versions {
    if mc == mc_version.0 {
      mc_forge_version.push(forge.to_owned());
    }
  }

  Ok(mc_forge_version)
}
