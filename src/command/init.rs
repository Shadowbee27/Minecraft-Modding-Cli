use clap::clap_derive::*;
use color_eyre::Result;
use semver::Version;
use std::{
  fs::{self, exists, read_dir},
  path::PathBuf,
};
use strum::IntoEnumIterator;

use crate::{
  config::{PROJECT_CONFIG_VERSION, ProjectConfig, mod_index::ModsIndex},
  mod_source::index::index_mods,
  software::{McVersion, ServerSoftware},
  tui::dialogs::{minecraft::mc_version_select, software::server_software_selector},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct InitArgs {
  pub name: Option<PathBuf>,
}

pub fn init_dir(args: InitArgs) -> Result<()> {
  let dir = if let Some(dir) = args.name {
    dir
  } else {
    PathBuf::from(".")
  };

  let files = read_dir(&dir)?;

  let mut jar = Vec::new();
  let mut dirs = Vec::new();

  for path in files {
    if path.as_ref().unwrap().path().is_dir() {
      dirs.push(path.unwrap().path());
    } else if path
      .as_ref()
      .unwrap()
      .path()
      .to_str()
      .unwrap()
      .ends_with("jar")
    {
      jar.push(path.unwrap().path());
    }
  }

  dbg!(&jar);
  dbg!(&dirs);

  // scan for serversoftware

  let mut serversoftware: ServerSoftware;
  let mc_version: McVersion;
  let jar_file: String;

  if !(jar.len() == 1) {
    println!(
      "Found more than one jar file, please select the minecraft version and software yourself"
    );
    serversoftware = server_software_selector()?;
    mc_version = mc_version_select(&serversoftware)?;
    jar_file = String::new();
  } else {
    jar_file = jar.first().unwrap().to_str().unwrap().to_string();
    let detection_results = try_get_server_software(&jar_file)?;

    if let Some(serv_ver) = detection_results.0 {
      serversoftware = serv_ver;
    } else {
      serversoftware = server_software_selector()?;
    }

    if let Some(mc_ver) = detection_results.1 {
      mc_version = mc_ver;
    } else {
      mc_version = mc_version_select(&serversoftware)?;
    }
  }
  serversoftware.set_flavor_version(&mc_version)?;

  dbg!(&mc_version);
  dbg!(&serversoftware);

  // create project config

  let pconf = ProjectConfig {
    config_version: PROJECT_CONFIG_VERSION,
    flavor: serversoftware.clone(),
    flavor_version: String::new(),
    mc_version: mc_version.clone(),
    jar_file: jar_file.clone(),
  };
  fs::create_dir_all(dir.join(".mcmc/"))?;
  pconf.save(&dir)?;

  // index mods

  let mod_dir = &serversoftware.get_moddification_dir();

  if let Some(mod_dir) = mod_dir {
    let mut index = ModsIndex::new(PathBuf::from(dir.join(mod_dir)))?;

    if exists(dir.join(mod_dir))? {
      index_mods(&mut index)?;
      dbg!(&index);
      index.write_conf()?;
    }
  }

  Ok(())
}

/// This is not reliabil in any way and won't probably work at all
fn try_get_server_software(
  file_name: &String,
) -> Result<(Option<ServerSoftware>, Option<McVersion>)> {
  dbg!(&file_name);
  let serversoftwares = Vec::from_iter(ServerSoftware::iter());

  let mut found_software = Vec::new();

  for software in serversoftwares {
    if file_name.contains(software.as_str()) {
      found_software.push(software);
    }
  }

  let server_software = if !(found_software.len() == 1) {
    None
  } else {
    found_software.first().cloned()
  };

  dbg!(&server_software);

  let split_file_name = file_name.split("-");
  let mut mc_ver: Option<McVersion> = None;

  for part in split_file_name {
    if part.contains("mc") {
      let mut part = part.replace("mc", "");
      loop {
        if part.chars().next().unwrap().is_numeric() {
          mc_ver = match Version::parse(&part) {
            Ok(v) => Some(McVersion(v.to_string())),
            Err(_) => None,
          };
          break;
        } else if part.is_empty() {
          break;
        } else {
          part = part.split_at(1).1.to_string();
        }
      }
    }
  }

  dbg!(&mc_ver);
  Ok((server_software, mc_ver))
}
