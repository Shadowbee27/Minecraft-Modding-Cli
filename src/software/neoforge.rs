use color_eyre::eyre::eyre;
use color_eyre::{Result, eyre::OptionExt};
use colorful::{Color, Colorful};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::runtime::Runtime;

use super::{ServerSoftware, Software, SupportedVersion};
use crate::common::maven::get_versions as get_maven_versions;
use crate::common::reqwest::download::download_file;
use crate::config::project_conf::ProjectConfig;
use crate::software::McVersion;
use crate::tui::dialogs::flavor::select_flavor_version;

const NEOFORGE_URL: &str = "https://maven.neoforged.net/releases";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeoForge {
  pub flavor_version: String,
}

impl Software for NeoForge {
  const NAME: &str = "NeoForged";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    self.get_self_versions(&pconf.mc_version)?;

    // https://maven.neoforged.net/releases/net/neoforged/neoforge/26.2.0.72/neoforge-26.2.0.72-installer.jar
    let url = format!(
      "{}/net/neoforged/neoforge/{}/neoforge-{}-installer.jar",
      NEOFORGE_URL, self.flavor_version, self.flavor_version
    );
    pconf.jar_file = PathBuf::from(format!(
      "{}/neoforge-{}-installer.jar",
      pdir.display(),
      self.flavor_version
    ));

    Runtime::new()?.block_on(download_file(
      &Client::new(),
      url.as_str(),
      &pconf.jar_file.display().to_string(),
    ))?;

    // java -jar test_server/neoforge-26.2-65.1.3-installer.jar --installServer test_server
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

    pconf.flavor_version = self.flavor_version.clone();
    pconf.flavor = ServerSoftware::NEOFORGE(self.clone());
    pconf.save(&Some(pdir.to_path_buf()))?;

    println!("----- NeoForge NeoForge-Installer output -----");
    let result = cmd.wait_with_output()?;

    let eoo = "----End of NeoForge-Installer output----";

    if !result.status.success() {
      println!("{}", eoo.color(Color::Red).bold());
      return Err(eyre!("Failed to run NeoForge-Installer, {}", result.status));
    }

    println!("{}", eoo.color(Color::Green).bold());
    Ok(())
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    let versions = get_neoforge_versions()?;

    let mut neoversions: Vec<SupportedVersion> = convert_neo_maven_version(versions)?
      .iter()
      .map(|v| v.0.clone())
      .collect();
    neoversions.dedup();
    neoversions.reverse();

    Ok(neoversions)
  }

  fn get_self_versions(&mut self, mc_version: &McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    self.flavor_version = select_flavor_version(get_neo_version_for_mc_version(mc_version)?)?;
    Ok(())
  }
}

fn get_neoforge_versions() -> Result<Vec<String>> {
  let rt = Runtime::new().unwrap();
  let meta = rt.block_on(get_maven_versions("net/neoforged/neoforge", NEOFORGE_URL))?;
  Ok(meta.versioning.versions.list)
}

fn convert_neo_maven_version(neo_versions: Vec<String>) -> Result<Vec<(SupportedVersion, String)>> {
  // This was done with help with AI
  let re = Regex::new(r"^(\d+)\.(\d+)(?:\.(\d+))?").unwrap();
  let mut supported_version: Vec<(SupportedVersion, String)> = Vec::new();

  for ver in neo_versions {
    if let Some(caps) = re.captures(ver.as_str()) {
      let major: u32 = caps[1].parse().unwrap();
      let minor: u32 = caps[2].parse().unwrap();
      let patch: u32 = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap_or(0));

      let stable = !(ver.contains("snapshot") || ver.contains("w"));

      let mc_version = if ver.contains("w") {
        // Split 0.25w14craftmine.4-beta so that we keep 25w14craftmine

        let (_, mc_snapshot) = ver
          .split_once('.')
          .ok_or_eyre(format!("Failed to split snapshot: {ver}"))?;
        let (mc_version_string, _) = mc_snapshot
          .split_once('.')
          .ok_or_eyre(format!("Failed to split snapshot: {ver}"))?;

        SupportedVersion::from_string_bool(mc_version_string.to_owned(), stable)
      } else if major >= 26 {
        // Minecraft 26+
        if patch > 0 {
          SupportedVersion::from_string_bool(format!("{}.{}.{}", major, minor, patch), stable)
        } else {
          SupportedVersion::from_string_bool(format!("{}.{}", major, minor), stable)
        }
      } else {
        // Legacy 1.X formatting (20.2 -> 1.20.2, 21.0 -> 1.21)
        if minor == 0 {
          SupportedVersion::from_string_bool(format!("1.{}", major), stable)
        } else {
          SupportedVersion::from_string_bool(format!("1.{}.{}", major, minor), stable)
        }
      };

      supported_version.push((mc_version, ver));
    }
  }

  supported_version.sort();
  supported_version.dedup();

  Ok(supported_version)
}

fn get_neo_version_for_mc_version(mc_version: &McVersion) -> Result<Vec<String>> {
  let neo_versions = get_neoforge_versions()?;

  let versions = convert_neo_maven_version(neo_versions)?;

  let mut neo_versions = Vec::new();
  for (mc_ver, neo_ver) in versions {
    if mc_ver.version == mc_version.0 {
      neo_versions.push(neo_ver);
    }
  }
  neo_versions.reverse();

  Ok(neo_versions)
}
