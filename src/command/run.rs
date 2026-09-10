use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::config::ProjectConfig;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct RunArgs {
  dir: Option<PathBuf>,
}

pub fn run_server(args: RunArgs) -> Result<()> {
  let project_config = ProjectConfig::load_config(&args.dir);
  unimplemented!()
}
