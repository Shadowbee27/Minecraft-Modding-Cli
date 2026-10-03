use color_eyre::eyre::eyre;
use color_eyre::{Result, eyre::OptionExt};
use colorful::{Color, Colorful};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

use super::SeverSoftware;
use super::SupportedVersion;
use crate::config::global_conf::get_buildtools_build_dir;
use crate::config::project_conf::ProjectConfig;
use crate::debug;
use crate::software::buildtools::{download_build_tools_jar, get_mc_versions};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bukkit {}

impl SeverSoftware for Bukkit {
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path) -> Result<()> {
    debug!(pdir);
    let path = format!(
      "{}/buildtools.jar",
      pdir
        .to_str()
        .ok_or_eyre("Failed to convert path to string")?
    );

    download_build_tools_jar(&path)?;

    let cmd = Command::new("java")
      .current_dir(get_buildtools_build_dir())
      .args([
        "-jar",
        path.as_str(),
        "--compile",
        "craftbukkit",
        "--o",
        pdir
          .to_str()
          .ok_or_eyre("Failed to convert path to string")?,
        "--rev",
        pconf.mc_version.0.as_str(),
      ])
      .spawn()?;

    println!("----Start of Buildtools output----");
    let result = cmd.wait_with_output()?;

    let eoo = "----End of Buildtools output----";

    if !result.status.success() {
      println!("{}", eoo.color(Color::Red).bold());
      return Err(eyre!("Failed to run Buildtools, {}", result.status));
    }

    println!("{}", eoo.color(Color::Green).bold());
    Ok(())
  }

  fn update(&mut self) -> Result<()> {
    unimplemented!()
  }

  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>> {
    get_mc_versions()
  }
}
