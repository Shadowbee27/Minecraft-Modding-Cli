use clap::clap_derive::*;
use color_eyre::{Result, eyre::ContextCompat};
// use semver::Version;
use std::{
  fs::{self, exists},
  path::PathBuf,
};
// use strum::IntoEnumIterator;
use walkdir::WalkDir;

use crate::{
  config::{
    mod_index::ModsIndex,
    project_conf::{PROJECT_CONFIG_VERSION, ProjectConfig},
  },
  debug,
  mod_source::index::index_mods,
  software::{McVersion, ServerSoftware},
  tui::{
    dialogs::{minecraft::mc_version_select, software::server_software_selector},
    modals::run::Ram,
  },
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct InitArgs {
  /// Directory to init, default "."
  pub dir: Option<PathBuf>,
}

pub fn init_dir(args: InitArgs) -> Result<()> {
  let dir = if let Some(dir) = args.dir {
    dir
  } else {
    PathBuf::from(".")
  };

  let mut jar = Vec::new();
  let mut dirs = Vec::new();

  for p in WalkDir::new(&dir).into_iter().flatten() {
    if p.file_type().is_dir() {
      dirs.push(p.into_path());
    } else if p.file_name().display().to_string().ends_with(".jar") {
      jar.push(p.into_path());
    }
  }

  debug!(&jar);
  debug!(&dirs);

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
    jar_file = jar.first().unwrap().display().to_string();
    // let detection_results = try_get_server_software(&jar_file)?;
    let detection_results = (None, None);

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

  debug!(&mc_version);
  debug!(&serversoftware);

  // create project config

  let pconf = ProjectConfig {
    server_name: dir
      .iter()
      .next_back()
      .wrap_err("Failed to get directory name")?
      .display()
      .to_string(),
    config_version: PROJECT_CONFIG_VERSION,
    flavor: serversoftware.clone(),
    flavor_version: String::new(),
    mc_version: mc_version.clone(),
    jar_file: PathBuf::from(jar_file.clone()),
    ram: Ram::default(),
    java_install: None,
  };
  fs::create_dir_all(dir.join(".mcmc/"))?;
  pconf.save(&Some(dir.clone()))?;

  // index mods

  let mod_dir = &serversoftware.get_moddification_dir();

  if let Some(mod_dir) = mod_dir {
    let mut index = ModsIndex::new(dir.join(mod_dir))?;

    if exists(dir.join(mod_dir))? {
      index_mods(&mut index)?;
      debug!(&index);
      index.write_conf()?;
    }
  }

  Ok(())
}

///// This is not reliabil in any way and won't probably work at all
// fn try_get_server_software(
//   file_name: &String,
// ) -> Result<(Option<ServerSoftware>, Option<McVersion>)> {
//   debug!(&file_name);
//   let serversoftwares = Vec::from_iter(ServerSoftware::iter());
//
//   let mut found_software = Vec::new();
//
//   for software in serversoftwares {
//     if file_name.contains(software.as_str()) {
//       found_software.push(software);
//     }
//   }
//
//   let server_software = if !(found_software.len() == 1) {
//     None
//   } else {
//     found_software.first().cloned()
//   };
//
//   debug!(&server_software);
//
//   let split_file_name = file_name.split("-");
//   let mut mc_ver: Option<McVersion> = None;
//
//   for part in split_file_name {
//     if part.contains("mc") {
//       let mut part = part.replace("mc", "");
//       loop {
//         if part.chars().next().unwrap().is_numeric() {
//           mc_ver = match Version::parse(&part) {
//             Ok(v) => Some(McVersion::form_string(v.to_string())?),
//
//             Err(_) => None,
//           };
//           break;
//         } else if part.is_empty() {
//           break;
//         } else {
//           part = part.split_at(1).1.to_string();
//         }
//       }
//     }
//   }
//
//   debug!(&mc_ver);
//   Ok((server_software, mc_ver))
// }
