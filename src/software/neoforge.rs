use color_eyre::eyre::OptionExt;
use color_eyre::eyre::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::runtime::Runtime;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::common::maven::get_versions as get_maven_versions;
use crate::config::ProjectConfig;

const NEOFORGE_URL: &str = "https://maven.neoforged.net/releases";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeoForge {
  pub flavor_version: String,
}

impl SeverSoftware for NeoForge {
  fn install(&mut self, _pconf: ProjectConfig, _pdir: &Path) -> Result<()> {
    //https://maven.neoforged.net/releases/net/neoforged/neoforge/26.2.0.72/neoforge-26.2.0.72-installer.jar
    unimplemented!()
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    let neo_versions = get_neoforge_versions()?;

    convert_neo_maven_version(neo_versions)
  }
}

fn get_neoforge_versions() -> Result<Vec<String>> {
  let rt = Runtime::new().unwrap();
  let meta = rt.block_on(get_maven_versions("net/neoforged/neoforge", NEOFORGE_URL))?;
  Ok(meta.versioning.versions.list)
}

fn convert_neo_maven_version(neo_versions: Vec<String>) -> Result<Vec<SupportedVersion>> {
  // This was done with help with AI
  let re = Regex::new(r"^(\d+)\.(\d+)(?:\.(\d+))?").unwrap();
  let mut supported_version: HashMap<SupportedVersion, String> = HashMap::new();

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

        SupportedVersion {
          version: mc_version_string.to_owned(),
          stable,
        }
      } else if major >= 26 {
        // Minecraft 26+
        if patch > 0 {
          SupportedVersion {
            version: format!("{}.{}.{}", major, minor, patch),
            stable,
          }
        } else {
          SupportedVersion {
            version: format!("{}.{}", major, minor),
            stable,
          }
        }
      } else {
        // Legacy 1.X formatting (20.2 -> 1.20.2, 21.0 -> 1.21)
        if minor == 0 {
          SupportedVersion {
            version: format!("1.{}", major),
            stable,
          }
        } else {
          SupportedVersion {
            version: format!("1.{}.{}", major, minor),
            stable,
          }
        }
      };

      supported_version.entry(mc_version.clone()).or_insert(ver);
    }
  }

  let mut mc_versions = Vec::new();
  for (mc_ver, _) in supported_version {
    mc_versions.push(mc_ver);
  }
  mc_versions.sort();
  Ok(mc_versions)
}
