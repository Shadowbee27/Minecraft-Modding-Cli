use color_eyre::Result;
use reqwest::Client as AClient;
use reqwest::blocking::Client as BClient;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::runtime::Runtime;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::common::download::download_file;
use crate::config::project_conf::ProjectConfig;
use crate::tui::dialogs::minecraft::mc_version_select;

const VANILLA_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vanilla {
  pub flavor_version: String,
}

#[derive(Debug, Clone, Deserialize)]
struct VanillaVersionMetadata {
  downloads: VanillaVersionDownloadMetadata,
}

#[derive(Debug, Clone, Deserialize)]
struct VanillaVersionDownloadMetadata {
  server: VanillaServerDownloadMetadata,
}

#[derive(Debug, Clone, Deserialize)]
struct VanillaServerDownloadMetadata {
  url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaMetadata {
  pub versions: Vec<VanillaVersions>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaVersions {
  pub id: String,
  #[serde(rename(deserialize = "type"))]
  pub version_type: String,
  pub url: String,
  #[serde(rename(deserialize = "releaseTime"))]
  pub release_time: String,
}

impl VanillaMetadata {
  fn to_supported_version(&self) -> Vec<SupportedVersion> {
    let mut supported_versions = Vec::new();

    for v in self.versions.clone() {
      let stable = v.version_type == "release";

      supported_versions.push(SupportedVersion::from_string_bool(v.id, stable));
    }
    supported_versions
  }
}

impl SeverSoftware for Vanilla {
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    let metadata = get_vanilla_metadata()?;
    let mut info_url = String::new();
    for m in metadata.versions {
      if m.id == pconf.mc_version.0 {
        info_url = m.url;
        break;
      }
    }

    let info = get_vanilla_version_metadata(&info_url)?;

    let client = AClient::new();
    let download_url = info.downloads.server.url;

    let file_path = format!("vanilla-server-mc{}.jar", pconf.mc_version.0);

    let rt = Runtime::new().unwrap();

    let result = rt.block_on(download_file(
      &client,
      &download_url,
      &format!("{}/{}", pdir.to_str().unwrap(), file_path),
    ));

    self.flavor_version = pconf.mc_version.0.clone();
    pconf.flavor = super::ServerSoftware::VANILLA(self.clone());
    pconf.jar_file = file_path;
    pconf.flavor_version = self.flavor_version.clone();

    pconf.save(pdir)?;

    result
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    Ok(get_vanilla_metadata()?.to_supported_version())
  }

  fn get_self_versions(&mut self, _mc_version: &super::McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    self.flavor_version = mc_version_select(&super::ServerSoftware::VANILLA(Vanilla::default()))?.0;
    Ok(())
  }
}

pub fn get_vanilla_metadata() -> color_eyre::Result<VanillaMetadata> {
  let client = BClient::new();
  Ok(client.get(VANILLA_URL).send()?.json::<VanillaMetadata>()?)
}

fn get_vanilla_version_metadata(
  metadata_url: &String,
) -> color_eyre::Result<VanillaVersionMetadata> {
  let client = BClient::new();
  Ok(
    client
      .get(metadata_url)
      .send()?
      .json::<VanillaVersionMetadata>()?,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  const METADATA: &str =
    "https://piston-meta.mojang.com/v1/packages/dc69be58cf16ad99f4b1ae7360c9a29c8c819ca5/26.2.json";

  #[test]
  fn vanilla_metadata() {
    dbg!(get_vanilla_metadata()).unwrap();
  }

  #[test]
  fn vanilla_version_metadata() {
    dbg!(get_vanilla_version_metadata(&METADATA.to_string())).unwrap();
  }
}
