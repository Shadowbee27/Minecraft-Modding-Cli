use clap::clap_derive::*;
use color_eyre::Result;
use std::path::PathBuf;

use crate::{
  common::java_manager::{
    JavaVersion, eight::Java8, find::find_javas, seventeen::Java17, twenty_five::Java25,
    twenty_one::Java21,
  },
  config::{global_conf::Config, project_conf::ProjectConfig},
  tui::dialogs::java::{select_java, select_java_version, select_java_version_for_mc_version},
};

#[derive(clap::Subcommand, Debug, Clone)]
pub enum JavaManagerArgs {
  /// List all java installs
  List(()),
  /// Set the java used for a specific server
  Set(JavaManagerSetArgs),
  /// Set the default java
  SetDefault(()),
  /// Install a java version
  Install(()),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct JavaManagerSetArgs {
  /// What server's java install should be changed, can be omitted default "."
  pub path: Option<PathBuf>,
}

pub fn java_manager(args: JavaManagerArgs) -> Result<()> {
  match args {
    JavaManagerArgs::List(_) => java_manager_list()?,
    JavaManagerArgs::Set(java_manager_set_args) => java_manager_set(java_manager_set_args)?,
    JavaManagerArgs::SetDefault(_) => java_manager_set_default()?,
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

pub fn java_manager_set_default() -> Result<()> {
  let mut config = Config::load()?;
  let java_ver = select_java_version()?;
  let javas: Vec<JavaVersion> = find_javas()?
    .iter()
    .filter_map(|j| match j {
      JavaVersion::JAVA8(_) => {
        if java_ver.is_java8() {
          Some(j.clone())
        } else {
          None
        }
      }
      JavaVersion::JAVA17(_) => {
        if java_ver.is_java17() {
          Some(j.clone())
        } else {
          None
        }
      }
      JavaVersion::JAVA21(_) => {
        if java_ver.is_java21() {
          Some(j.clone())
        } else {
          None
        }
      }
      JavaVersion::JAVA25(_) => {
        if java_ver.is_java25() {
          Some(j.clone())
        } else {
          None
        }
      }
    })
    .collect();
  let java = select_java(javas)?;

  match java_ver {
    JavaVersion::JAVA8(_) => {
      config.java_settings.default_8 = java
        .to_java()
        .as_any()
        .downcast_ref::<Java8>()
        .unwrap()
        .clone()
    }
    JavaVersion::JAVA17(_) => {
      config.java_settings.default_17 = java
        .to_java()
        .as_any()
        .downcast_ref::<Java17>()
        .unwrap()
        .clone()
    }
    JavaVersion::JAVA21(_) => {
      config.java_settings.default_21 = java
        .to_java()
        .as_any()
        .downcast_ref::<Java21>()
        .unwrap()
        .clone()
    }
    JavaVersion::JAVA25(_) => {
      config.java_settings.default_25 = java
        .to_java()
        .as_any()
        .downcast_ref::<Java25>()
        .unwrap()
        .clone()
    }
  }

  config.save()?;
  Ok(())
}

pub fn java_manager_install() -> Result<()> {
  JavaVersion::install(select_java_version()?)
}
