use clap::clap_derive::*;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct InitArgs {
  pub name: PathBuf,
}

pub fn init_dir(args: InitArgs) {
  todo!()
}
