use clap::clap_derive::*;
use std::fs::{create_dir_all, exists};
use std::{path::PathBuf, process::exit};

use crate::config::{CONFIG_VERSION, ProjectConfig};
use crate::software::{ServerSoftware, SeverSoftware};
use crate::tui::dialogs::minecraft::mc_version_select;
use crate::tui::dialogs::software::server_software_selector;

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

pub fn create_new_server(args: CreateNewArgs) {
  if exists(&args.name).unwrap() {
    println!("Directory is not empty, please use init");
    exit(1);
  }

  let mut project_config: ProjectConfig;

  let server_software: ServerSoftware = server_software_selector();
  let modding_folder = server_software.get_moddification_dir();

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

  if modding_folder.is_some() {
    if modding_folder.unwrap() == "mods" {
      dirs_to_create.push(ServerDir {
        dir: PathBuf::from(&args.name.join(modding_folder.unwrap())),
        creation_error: "Failed to create mods directory:",
      });
    } else if modding_folder.unwrap() == "plugins" {
      dirs_to_create.push(ServerDir {
        dir: PathBuf::from(&args.name.join(modding_folder.unwrap())),
        creation_error: "Failed to create plugins directory:",
      });
    }
  }

  for dir in dirs_to_create {
    match create_dir_all(dir.dir) {
      Ok(_) => {}
      Err(e) => eprintln!("{} ({})", dir.creation_error, e),
    }
  }
  let minecraft_version = mc_version_select();
  project_config = ProjectConfig {
    config_version: CONFIG_VERSION,
    flavor: server_software.clone(),
    flavor_version: server_software.clone().get_flavor_version(),
    mc_version: minecraft_version.clone(),
  };

  project_config = match project_config.write_to_file(&args.name) {
    Ok(conf) => conf,
    Err(e) => {
      eprintln!("Error while saving config file: ({e})");
      exit(1)
    }
  };

  // Todo: Error handling, but it is blocked by the reason that it is not know which errors need to be handled
  let install_error = match server_software {
    ServerSoftware::FABRIC(f) => f.install(project_config),
    ServerSoftware::FOLIA(f) => f.install(project_config),
    ServerSoftware::FORGE(f) => f.install(project_config),
    ServerSoftware::NEOFORGE(n) => n.install(project_config),
    ServerSoftware::PAPER(p) => p.install(project_config),
    ServerSoftware::VANILLA(v) => v.install(project_config),
  };

  println!("Success, new server created");
}
