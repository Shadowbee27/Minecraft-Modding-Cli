use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::{
  common::java_manager::{JavaVersion, find::find_javas},
  config::project_conf::ProjectConfig,
  tui::dialogs::java::{select_java_version, select_java_version_for_mc_version},
};

#[derive(clap::Subcommand, Debug, Clone)]
pub enum JavaManagerArgs {
  /// List all java installs
  List(()),
  /// Set the java used for a specific server
  Set(JavaManagerSetArgs),
  /// Set the default java
  SetDefault(JavaManagerSetDefaultArgs),
  /// Install a java version
  Install(()),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct JavaManagerSetArgs {
  /// What server's java install should be changed, can be omitted default "."
  pub path: Option<PathBuf>,
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

pub fn java_manager_set(args: JavaManagerSetArgs) -> Result<()> {
  let mut pconf = ProjectConfig::load_project_config(&args.path)?;
  let javas = find_javas()?;

  pconf.java_install = Some(select_java_version_for_mc_version(
    javas,
    &pconf.mc_version,
  )?);

  pconf.save(&None)?;
  Ok(())
}

pub fn java_manager_set_default(_args: JavaManagerSetDefaultArgs) -> Result<()> {
  unimplemented!()
}

pub fn java_manager_install() -> Result<()> {
  JavaVersion::install(select_java_version()?)
}
