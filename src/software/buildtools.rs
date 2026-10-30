use color_eyre::Result;
use rayon::prelude::*;
use reqwest::Client;
use tokio::runtime::Runtime;

use crate::{
  common::{maven::get_versions, reqwest::download::download_file},
  software::SupportedVersion,
};

const DOWNLOAD_URL: &str =
  "https://hub.spigotmc.org/jenkins/job/BuildTools/lastBuild/artifact/target/BuildTools.jar";

pub fn download_build_tools_jar(path: &str) -> Result<()> {
  Runtime::new()?.block_on(download_file(
    &Client::builder().user_agent("curl/8.23.0").build()?,
    DOWNLOAD_URL,
    path,
  ))
}

pub fn get_mc_versions() -> Result<Vec<SupportedVersion>> {
  // https://hub.spigotmc.org/nexus/repository/public/org/spigotmc/spigot-api/maven-metadata.xml
  let versions = Runtime::new()?
    .block_on(get_versions(
      "org/spigotmc/spigot-api",
      "https://hub.spigotmc.org/nexus/repository/public",
    ))?
    .versioning
    .versions
    .list;

  let versions = versions
    .par_iter()
    .map(|v| {
      v.split_once("-")
        .expect("Failed to split Bukkit/Spigot version")
        .0
    })
    .collect::<Vec<&str>>();

  let mut supported_ver = Vec::new();
  for ver in versions {
    supported_ver.push(SupportedVersion {
      version: ver.to_owned(),
      stable: true,
    });
  }
  supported_ver.reverse();
  Ok(supported_ver)
}
