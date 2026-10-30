use color_eyre::eyre::eyre;
use color_eyre::{Result, eyre::OptionExt};
use colorful::{Color, Colorful};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

use super::Software;
use super::SupportedVersion;
use crate::config::global_conf::get_buildtools_build_dir;
use crate::config::project_conf::ProjectConfig;
use crate::debug;
use crate::software::ServerSoftware;
use crate::software::buildtools::{download_build_tools_jar, get_mc_versions};
use crate::tui::common::simple_prompt::yes_no_prompt;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::update::{WhatToUpdate, what_to_update};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bukkit {}

impl Software for Bukkit {
  const NAME: &str = "Bukkit";
  fn install(&mut self, mut pconf: ProjectConfig, pdir: &Path) -> Result<()> {
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

    pconf.jar_file = PathBuf::from(pdir).join(format!("craftbukzkit-{}.jar", pconf.mc_version.0));
    pconf.save(&Some(pdir.to_path_buf()))?;

    println!("{}", eoo.color(Color::Green).bold());
    Ok(())
  }

  fn update(&mut self, mut pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    let bukkit = ServerSoftware::BUKKIT(Bukkit::default());
    match what_to_update(&bukkit)? {
      WhatToUpdate::MINECRAFT => {
        let mc_ver = mc_version_select(&bukkit)?;
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
    get_mc_versions()
  }
}
