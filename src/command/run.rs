use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::config::project_conf::ProjectConfig;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct RunArgs {
  /// The directory that contains the server, default "."
  dir: Option<PathBuf>,
}

pub fn run_server(args: RunArgs) -> Result<()> {
  let _project_config = ProjectConfig::load_project_config(&args.dir);
  unimplemented!()
}
