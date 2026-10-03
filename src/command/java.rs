use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::{
  common::java_manager::{JavaVersion, find::find_javas},
  tui::dialogs::java::select_java_version,
};

#[derive(clap::Subcommand, Debug, Clone)]
pub enum JavaManagerArgs {
  List(()),
  Set(JavaManagerSetArgs),
  SetDefault(JavaManagerSetDefaultArgs),
  Install(()),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct JavaManagerSetArgs {
  pub path: PathBuf,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct JavaManagerSetDefaultArgs {}

pub fn java_manager(args: JavaManagerArgs) -> Result<()> {
  match args {
    JavaManagerArgs::List(_) => java_manager_list()?,
    JavaManagerArgs::Set(java_manager_set_args) => java_manager_set(java_manager_set_args)?,
    JavaManagerArgs::SetDefault(java_manager_set_default_args) => {
      java_manager_set_default(java_manager_set_default_args)?
    }
    JavaManagerArgs::Install(_) => java_manager_install()?,
  }
  Ok(())
}

pub fn java_manager_list() -> Result<()> {
  let javas = find_javas()?;
  for j in javas {
    println!("{}", j);
  }

  Ok(())
}

pub fn java_manager_set(_args: JavaManagerSetArgs) -> Result<()> {
  unimplemented!()
}

pub fn java_manager_set_default(_args: JavaManagerSetDefaultArgs) -> Result<()> {
  unimplemented!()
}

pub fn java_manager_install() -> Result<()> {
  JavaVersion::install(select_java_version()?)
}
