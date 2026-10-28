use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::{
  config::project_conf::ProjectConfig,
  software::ServerDetails,
  tui::{self, modals::run::ExeArgs},
};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct RunArgs {
  /// The directory that contains the server, default "."
  dir: Option<PathBuf>,
}

pub fn run_server(args: RunArgs) -> Result<()> {
  let project_config = ProjectConfig::load_project_config(&args.dir)?;
  let run_dir;
  if let Some(dir) = args.dir {
    run_dir = dir;
  } else {
    run_dir = PathBuf::from(".");
  }
  tui::modals::run::init_run_server_modal(
    ExeArgs {
      mc_ver: project_config.mc_version.clone(),
      java: project_config.java_install.clone(),
      name: project_config.server_name.clone(),
      jar: project_config.jar_file.clone().display().to_string(),
      ram: project_config.ram,
      path: run_dir,
    },
    ServerDetails::from_software(project_config),
  )
}
