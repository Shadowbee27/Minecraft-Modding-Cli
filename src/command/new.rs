use clap::clap_derive::*;
use colorful::{Color, Colorful};
use std::fs::{create_dir_all, exists};
use std::{path::PathBuf, process::exit};

use crate::config::{CONFIG_VERSION, ProjectConfig};
use crate::software::ServerSoftware;
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

  let server_software: ServerSoftware = server_software_selector();

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
    match create_dir_all(dir.dir) {
      Ok(_) => {}
      Err(e) => eprintln!(
        "{} ({})",
        dir.creation_error,
        e.to_string().color(Color::Red).bold()
      ),
    }
  }

  let project_config = ProjectConfig {
    config_version: CONFIG_VERSION,
    flavor: server_software.clone(),
    flavor_version: server_software.clone().get_flavor_version(),
    mc_version: minecraft_version.clone(),
    jar_file: String::new(),
  };

  match project_config.save(&args.name) {
    Ok(_) => {}
    Err(e) => {
      eprintln!(
        "Error while saving config file: ({})",
        e.to_string().color(Color::Red).bold()
      );
      exit(1)
    }
  };

  server_software.install(project_config, &args.name);

  println!("Success, new server created");
}
