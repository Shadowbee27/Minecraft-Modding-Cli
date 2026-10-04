use clap::clap_derive::*;
use color_eyre::{
  Result,
  eyre::{Context, eyre},
};
use std::fs::{create_dir_all, exists};
use std::path::PathBuf;

use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::software::server_software_selector;
use crate::{
  common::get_current_full_dir,
  config::project_conf::{PROJECT_CONFIG_VERSION, ProjectConfig},
};
use crate::{config::project_conf::LaunchArgs, software::ServerSoftware};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct CreateNewArgs {
  /// The directory of the new server
  pub dir: PathBuf,
}

#[derive(Debug, Clone)]
struct ServerDir<'a> {
  dir: PathBuf,
  creation_error: &'a str,
}

pub fn create_new_server(mut args: CreateNewArgs) -> Result<()> {
  args.dir = get_current_full_dir()?.join(args.dir); // use full instedt of relitiv path
  if exists(&args.dir)? {
    return Err(eyre!("Directory is not empty, please use init"));
  }

  let mut server_software: ServerSoftware = server_software_selector()?;

  let modding_folder = server_software.get_moddification_dir();

  let minecraft_version = mc_version_select(&server_software);

  let mut dirs_to_create: Vec<ServerDir> = vec![
    ServerDir {
      dir: args.dir.clone(),
      creation_error: "Error while creating server directory:",
    },
    ServerDir {
      dir: PathBuf::from(&args.dir.join(".mcmc")),
      creation_error: "Error while creating mcmc directory:",
    },
  ];

  if let Some("mods/") = modding_folder {
    dirs_to_create.push(ServerDir {
      dir: PathBuf::from(&args.dir.join(modding_folder.unwrap())),
      creation_error: "Failed to create mods directory:",
    });
  }
  if let Some("plugins/") = modding_folder {
    dirs_to_create.push(ServerDir {
      dir: PathBuf::from(&args.dir.join(modding_folder.unwrap())),
      creation_error: "Failed to create plugins directory:",
    });
  }

  for dir in dirs_to_create {
    create_dir_all(dir.dir).wrap_err_with(|| dir.creation_error.to_string())?;
  }

  let project_config = ProjectConfig {
    config_version: PROJECT_CONFIG_VERSION,
    flavor: server_software.clone(),
    flavor_version: server_software.clone().get_flavor_version(),
    mc_version: minecraft_version?.clone(),
    jar_file: String::new(),
    launch_args: LaunchArgs::default(),
    java_install: None,
  };

  project_config
    .save(&Some(args.dir.clone()))
    .wrap_err("Error while saving project-config file")?;

  server_software.install(project_config, &args.dir)?;

  println!("Success, new server created");

  Ok(())
}
