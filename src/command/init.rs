use clap::clap_derive::*;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct InitArgs {
  pub name: PathBuf,
}

pub fn init_dir(_args: InitArgs) {
  // scan for serversoftware
  // index mods
  // create mods

  todo!()
}
