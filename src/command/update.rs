use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::config::project_conf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct UpdateArgs {
  /// Which server should be updated, can be omitted default "."
  pub path: Option<PathBuf>,
}

pub fn update(args: UpdateArgs) -> Result<()> {
  let pconf = project_conf::ProjectConfig::load_project_config(&args.path)?;

  pconf.flavor.clone().update(pconf, args.path)
}
