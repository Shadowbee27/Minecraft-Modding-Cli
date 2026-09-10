use clap::clap_derive::*;
use color_eyre::{
  Result,
  eyre::{Context, eyre},
};
use std::fs::{create_dir_all, exists};
use std::path::PathBuf;

use crate::software::ServerSoftware;
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::software::server_software_selector;
use crate::{
  common::get_current_full_dir,
  config::{ProjectConfig, project_conf::PROJECT_CONFIG_VERSION},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct CreateNewArgs {
  pub name: PathBuf,
}

#[derive(Debug, Clone)]
struct ServerDir<'a> {
  dir: PathBuf,
  creation_error: &'a str,
}

pub fn create_new_server(mut args: CreateNewArgs) -> Result<()> {
  args.name = get_current_full_dir()?.join(args.name); // use full instedt of relitiv path
  if exists(&args.name)? {
    return Err(eyre!("Directory is not empty, please use init"));
  }

  let mut server_software: ServerSoftware = server_software_selector()?;

  let modding_folder = server_software.get_moddification_dir();

  let minecraft_version = mc_version_select(&server_software);

  let mut dirs_to_create: Vec<ServerDir> = vec![
    ServerDir {
      dir: args.name.clone(),
      creation_error: "Error while creating server directory:",
    },
    ServerDir {
      dir: PathBuf::from(&args.name.join(".mcmc")),
      creation_error: "Error while creating mcmc directory:",
    },
  ];

  if let Some("mods/") = modding_folder {
    dirs_to_create.push(ServerDir {
      dir: PathBuf::from(&args.name.join(modding_folder.unwrap())),
      creation_error: "Failed to create mods directory:",
    });
  }
  if let Some("plugins/") = modding_folder {
    dirs_to_create.push(ServerDir {
      dir: PathBuf::from(&args.name.join(modding_folder.unwrap())),
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
  };

  project_config
    .save(&args.name)
    .wrap_err("Error while saving project-config file")?;

  server_software.install(project_config, &args.name)?;

  println!("Success, new server created");

  Ok(())
}
