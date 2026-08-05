use std::path::Path;
use std::process::exit;

use super::SeverSoftware;
use crate::{common::download::download_file, config::ProjectConfig};
use reqwest::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

const FABRIC_URL: &str = "https://meta.fabricmc.net";
const FABRIC_INFO_URL: &str = "/v2/versions/loader";
const FABRIC_DOWNLOAD_URL: &str = "/v2/versions/loader/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fabric {
  pub flavor_version: Version,
  pub loader_version: Version,
}

impl SeverSoftware for Fabric {
  fn install(&self, pconf: ProjectConfig, pdir: &Path) {
    let client = Client::new();
    // BSP: https://meta.fabricmc.net/v2/versions/loader/26.2/0.19.3/0.8.0/server/jar
    let download_url = format!(
      "{}{}{}/{}/{}/server/jar",
      FABRIC_URL,
      FABRIC_DOWNLOAD_URL,
      &pconf.mc_version.0.to_string(),
      &pconf.flavor_version.to_string(),
      &self.loader_version.to_string()
    );

    let file_path = format!(
      "{}/fabric-server-mc{}-fabric{}.jar",
      pdir.to_str().unwrap(),
      &pconf.mc_version.0.to_string(),
      &pconf.flavor_version.to_string()
    );

    let rt = Runtime::new().unwrap();

    let result = rt.block_on(download_file(&client, &download_url, &file_path));

    match result {
      Ok(_) => {}
      Err(e) => {
        eprintln!("Error while downloading file: ({e})");
        exit(1)
      }
    };
  }

  fn update(&self) {
    unimplemented!()
  }
}
