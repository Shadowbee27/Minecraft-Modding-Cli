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

  let server_software: ServerSoftware = server_software_selector();
  // let server_software: ServerSoftware = ServerSoftware::FABRIC(Fabric {
  //   flavor_version: Version {
  //     major: 0,
  //     minor: 19,
  //     patch: 3,
  //     pre: Prerelease::EMPTY,
  //     build: BuildMetadata::EMPTY,
  //   },
  //   loader_version: Version {
  //     major: 1,
  //     minor: 1,
  //     patch: 2,
  //     pre: Prerelease::EMPTY,
  //     build: BuildMetadata::EMPTY,
  //   },
  // });
  let modding_folder = server_software.get_moddification_dir();
  // let modding_folder = Some("mods/");

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
      Err(e) => eprintln!("{} ({})", dir.creation_error, e),
    }
  }
  let minecraft_version = mc_version_select();
  // let minecraft_version = McVersion(Version {
  //   major: 26,
  //   minor: 1,
  //   patch: 1,
  //   pre: Prerelease::EMPTY,
  //   build: BuildMetadata::EMPTY,
  // });
  let project_config = ProjectConfig {
    config_version: CONFIG_VERSION,
    flavor: server_software.clone(),
    flavor_version: server_software.clone().get_flavor_version(),
    mc_version: minecraft_version.clone(),
  };

  match project_config.write_to_file(&args.name) {
    Ok(_) => {}
    Err(e) => {
      eprintln!("Error while saving config file: ({e})");
      exit(1)
    }
  };

  match server_software {
    ServerSoftware::FABRIC(f) => f.install(project_config, &args.name),
    ServerSoftware::FOLIA(f) => f.install(project_config, &args.name),
    ServerSoftware::FORGE(f) => f.install(project_config, &args.name),
    ServerSoftware::NEOFORGE(n) => n.install(project_config, &args.name),
    ServerSoftware::PAPER(p) => p.install(project_config, &args.name),
    ServerSoftware::VANILLA(v) => v.install(project_config, &args.name),
  };

  println!("Success, new server created");
}
