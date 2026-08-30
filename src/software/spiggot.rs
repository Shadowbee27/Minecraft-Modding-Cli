use color_eyre::{Result, eyre::OptionExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::ProjectConfig;
use crate::software::buildtools::{download_build_tools_jar, get_mc_versions};

// const _SPIGGOT_URL: &str = "hub.spigotmc.org";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Spiggot {}

impl SeverSoftware for Spiggot {
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    let path = format!(
      "{}/buildtools.jar",
      pdir
        .to_str()
        .ok_or_eyre("Failed to convert path to string")?
    );

    download_build_tools_jar(&path)?;

    let cmd = Command::new("java")
      .args([
        "-jar",
        path.as_str(),
        "--o",
        pdir
          .to_str()
          .ok_or_eyre("Failed to convert path to string")?,
        "--rev",
        pconf.mc_version.0.as_str(),
      ])
      .spawn()?;

    println!("----Start of Buildtools output----");
    cmd.wait_with_output()?;
    println!("----End of Buildtools output----");

    Ok(())
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    get_mc_versions()
  }
}
