use clap::clap_derive::*;
use std::fs::{create_dir_all, exists};
use std::{path::PathBuf, process::exit};

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

  let dirs_to_create: [ServerDir; 3] = [
    ServerDir {
      dir: args.name,
      creation_error: "Error while creating server directory:",
    },
    ServerDir {
      dir: PathBuf::from(""),
      creation_error: "Error while creating mods directory:",
    },
    ServerDir {
      dir: PathBuf::from(".mcmc"),
      creation_error: "Error while creating mcmc directory:",
    },
  ];

  for dir in dirs_to_create {
    match create_dir_all(dir.dir) {
      Ok(_) => {}
      Err(e) => eprintln!("{} ({})", dir.creation_error, e),
    }
  }
  unimplemented!()
}
