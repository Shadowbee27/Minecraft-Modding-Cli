use color_eyre::{
  Result,
  eyre::{ContextCompat, Ok, eyre},
};
use date::Date;
use serde::{Deserialize, Serialize};
use std::{
  fmt::{self, Display},
  path::{Path, PathBuf},
  str::FromStr,
};
use strum::EnumIter;

use crate::{
  common::java_manager::JavaVersion,
  config::{global_conf::Config, project_conf::ProjectConfig},
  debug,
  software::{
    babric::Babric,
    bukkit::Bukkit,
    fabric::Fabric,
    folia::Folia,
    forge::Forge,
    legacy_fabric::LegacyFabric,
    neoforge::NeoForge,
    paper::Paper,
    purpur::Purpur,
    quilt::Quilt,
    spiggot::Spiggot,
    vanilla::{Vanilla, get_vanilla_metadata},
  },
};

pub mod babric;
mod buildtools;
pub mod bukkit;
pub mod fabric;
pub mod folia;
pub mod forge;
pub mod legacy_fabric;
pub mod neoforge;
pub mod paper;
pub mod purpur;
pub mod quilt;
pub mod spiggot;
pub mod vanilla;

// Unlikely to be supported: Sponge, Java agent, liteloader, RisugamisModLoader, Nil loader, Ornithe, Rift
// Todo:
// install done: fabric, folia, paper, purpur, Legacy Fabric, vanilla, quilt, forge, neoforge, bukkit, spiggot, Babric
// update done:
#[derive(Debug, Clone, Deserialize, Serialize, EnumIter)]
pub enum ServerSoftware {
  BUKKIT(Bukkit),
  FABRIC(Fabric),
  FOLIA(Folia),
  FORGE(Forge),
  LegacyFabric(LegacyFabric),
  NEOFORGE(NeoForge),
  PAPER(Paper),
  PURPUR(Purpur),
  QUILT(Quilt),
  SPIGGOT(Spiggot),
  VANILLA(Vanilla),
  BABRIC(Babric),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SupportedVersion {
  pub version: String, // semver can't be used thx to minecraft
  pub stable: bool,
}

impl SupportedVersion {
  pub fn from_string_bool(ver: String, stable: bool) -> Self {
    debug!(&ver);
    SupportedVersion {
      version: ver,
      stable,
    }
  }
}

impl std::fmt::Display for SupportedVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.version)
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McVersion(pub String);

impl PartialEq for McVersion {
  fn eq(&self, other: &Self) -> bool {
    McVersion::get_date(&self.0).unwrap() == McVersion::get_date(&other.0).unwrap()
  }
}

impl Eq for McVersion {}

impl PartialOrd for McVersion {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for McVersion {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    McVersion::get_date(&self.0)
      .unwrap()
      .cmp(&McVersion::get_date(&other.0).unwrap())
  }
}

impl McVersion {
  pub fn get_default_java(mc_version: &McVersion) -> Result<JavaVersion> {
    let config = Config::load()?;
    let date = McVersion::get_date(&mc_version.0)?;
    if date < Date::from_str("2021-05-12").unwrap() {
      // Java 8
      Ok(JavaVersion::JAVA8(config.java_settings.default_8))
    } else if date < Date::from_str("2021-09-15").unwrap() {
      // Java 16
      Ok(JavaVersion::JAVA17(config.java_settings.default_17))
    } else if date < Date::from_str("2024-04-03").unwrap() {
      // Java 17
      Ok(JavaVersion::JAVA17(config.java_settings.default_17))
    } else if date < Date::from_str("2025-12-16").unwrap() {
      // Java 21
      Ok(JavaVersion::JAVA21(config.java_settings.default_21))
    } else {
      // Java 25
      Ok(JavaVersion::JAVA25(config.java_settings.default_25))
    }
  }

  pub fn form_string(version: String) -> Result<Self> {
    Ok(McVersion(version))
  }

  pub fn get_date<T>(mc_version: &T) -> Result<Date>
  where
    str: PartialEq<T>,
  {
    let meta = get_vanilla_metadata()?;

    let mut date = Date::MIN;

    for v in meta.versions {
      debug!(&v);
      if v.id.as_str() == mc_version {
        let date_time = v
          .release_time
          .split_once("T")
          .wrap_err("Failed to parse Date")?
          .0;
        debug!(&date_time);
        date = Date::from_str(date_time)?;
        break;
      }
    }

    if date == Date::MIN {
      debug!(&date);
      return Err(eyre!("Unknown Minecraft version"));
    }

    Ok(date)
  }
}

pub trait Software {
  const NAME: &str;
  fn name(&self) -> &'static str {
    Self::NAME
  }
  fn update(&mut self, pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()>;
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path) -> Result<()>;
  fn get_supported_minecraft_versions() -> Result<Vec<SupportedVersion>>
  where
    Self: Sized;
  fn get_self_versions(&mut self, _mc_version: &McVersion) -> Result<()>
  where
    Self: Sized + Clone,
  {
    Ok(())
  }
}

impl ServerSoftware {
  pub fn update(self, pconf: ProjectConfig, pdir: Option<PathBuf>) -> Result<()> {
    match self {
      ServerSoftware::BUKKIT(mut bukkit) => bukkit.update(pconf, pdir),
      ServerSoftware::FABRIC(mut fabric) => fabric.update(pconf, pdir),
      ServerSoftware::FOLIA(mut folia) => folia.update(pconf, pdir),
      ServerSoftware::FORGE(mut forge) => forge.update(pconf, pdir),
      ServerSoftware::LegacyFabric(mut legacy_fabric) => legacy_fabric.update(pconf, pdir),
      ServerSoftware::NEOFORGE(mut neo_forge) => neo_forge.update(pconf, pdir),
      ServerSoftware::PAPER(mut paper) => paper.update(pconf, pdir),
      ServerSoftware::PURPUR(mut purpur) => purpur.update(pconf, pdir),
      ServerSoftware::QUILT(mut quilt) => quilt.update(pconf, pdir),
      ServerSoftware::SPIGGOT(mut spiggot) => spiggot.update(pconf, pdir),
      ServerSoftware::VANILLA(mut vanilla) => vanilla.update(pconf, pdir),
      ServerSoftware::BABRIC(mut babric) => babric.update(pconf, pdir),
    }
  }
  pub fn get_supported_minecraft_versions(&self) -> Result<Vec<SupportedVersion>> {
    match self {
      Self::FABRIC(_) => Fabric::get_supported_minecraft_versions(),
      Self::FOLIA(_) => Folia::get_supported_minecraft_versions(),
      Self::FORGE(_) => Forge::get_supported_minecraft_versions(),
      Self::NEOFORGE(_) => NeoForge::get_supported_minecraft_versions(),
      Self::PAPER(_) => Paper::get_supported_minecraft_versions(),
      Self::VANILLA(_) => Vanilla::get_supported_minecraft_versions(),
      Self::BUKKIT(_) => Bukkit::get_supported_minecraft_versions(),
      Self::QUILT(_) => Quilt::get_supported_minecraft_versions(),
      Self::LegacyFabric(_) => LegacyFabric::get_supported_minecraft_versions(),
      Self::PURPUR(_) => Purpur::get_supported_minecraft_versions(),
      Self::SPIGGOT(_) => Spiggot::get_supported_minecraft_versions(),
      Self::BABRIC(_) => Babric::get_supported_minecraft_versions(),
    }
  }

  pub const fn as_str<T: Software>() -> &'static str {
    T::NAME
  }

  pub fn install(&mut self, project_config: ProjectConfig, server_dir: &Path) -> Result<()> {
    match self {
      Self::FABRIC(f) => f.install(project_config, server_dir),
      Self::FOLIA(f) => f.install(project_config, server_dir),
      Self::FORGE(f) => f.install(project_config, server_dir),
      Self::NEOFORGE(n) => n.install(project_config, server_dir),
      Self::PAPER(p) => p.install(project_config, server_dir),
      Self::VANILLA(v) => v.install(project_config, server_dir),
      Self::BUKKIT(b) => b.install(project_config, server_dir),
      Self::QUILT(q) => q.install(project_config, server_dir),
      Self::LegacyFabric(l) => l.install(project_config, server_dir),
      Self::PURPUR(p) => p.install(project_config, server_dir),
      Self::SPIGGOT(s) => s.install(project_config, server_dir),
      Self::BABRIC(b) => b.install(project_config, server_dir),
    }
  }

  pub fn get_moddification_dir(&self) -> Option<&str> {
    match &self {
      Self::FABRIC(_)
      | Self::FORGE(_)
      | Self::NEOFORGE(_)
      | Self::QUILT(_)
      | Self::BABRIC(_)
      | Self::LegacyFabric(_) => Some("mods/"),
      Self::FOLIA(_) | Self::PAPER(_) | Self::BUKKIT(_) | Self::SPIGGOT(_) | Self::PURPUR(_) => {
        Some("plugins/")
      }
      Self::VANILLA(_) => None,
    }
  }

  pub fn get_flavor_version(self) -> String {
    match self {
      Self::FABRIC(f) => f.flavor_version.to_string(),
      Self::FOLIA(f) => f.flavor_version.to_string(),
      Self::FORGE(f) => f.flavor_version.to_string(),
      Self::NEOFORGE(n) => n.flavor_version.to_string(),
      Self::PAPER(p) => p.flavor_version.to_string(),
      Self::BUKKIT(_) => String::new(),
      Self::QUILT(_) => String::new(),
      Self::VANILLA(v) => v.flavor_version.to_string(),
      Self::LegacyFabric(l) => l.flavor_version.to_string(),
      Self::PURPUR(p) => p.flavor_version,
      Self::SPIGGOT(_) => String::new(),
      Self::BABRIC(b) => b.flavor_version.to_string(),
    }
  }

  pub fn set_flavor_version(&mut self, mc_version: &McVersion) -> Result<()> {
    match self {
      Self::FABRIC(f) => f.get_self_versions(mc_version)?,
      Self::FOLIA(f) => f.get_self_versions(mc_version)?,
      Self::FORGE(f) => f.get_self_versions(mc_version)?,
      Self::NEOFORGE(n) => n.get_self_versions(mc_version)?,
      Self::PAPER(p) => p.get_self_versions(mc_version)?,
      Self::BUKKIT(b) => b.get_self_versions(mc_version)?,
      Self::QUILT(q) => q.get_self_versions(mc_version)?,
      Self::VANILLA(v) => v.get_self_versions(mc_version)?,
      Self::LegacyFabric(l) => l.get_self_versions(mc_version)?,
      Self::PURPUR(p) => p.get_self_versions(mc_version)?,
      Self::SPIGGOT(s) => s.get_self_versions(mc_version)?,
      Self::BABRIC(b) => b.get_self_versions(mc_version)?,
    }

    Ok(())
  }
}

impl Display for ServerSoftware {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::FABRIC(_) => write!(f, "Fabric"),
      Self::FOLIA(_) => write!(f, "Folia"),
      Self::FORGE(_) => write!(f, "Forge"),
      Self::NEOFORGE(_) => write!(f, "Neoforge"),
      Self::PAPER(_) => write!(f, "Paper"),
      Self::VANILLA(_) => write!(f, "Vanilla"),
      Self::BUKKIT(_) => write!(f, "Bukkit"),
      Self::QUILT(_) => write!(f, "Quilt"),
      Self::LegacyFabric(_) => write!(f, "Legacy Fabric"),
      Self::PURPUR(_) => write!(f, "Purpur"),
      Self::SPIGGOT(_) => write!(f, "Spiggot"),
      Self::BABRIC(_) => write!(f, "Babric"),
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ServerDetails {
  pub name: Box<str>,
  pub version: Box<str>,
  pub flavor: Box<str>,
}

impl fmt::Display for ServerDetails {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      " Fabric: {} - {} - Minecraf: {} ",
      self.flavor, self.name, self.version
    )
  }
}

impl ServerDetails {
  pub fn from_software(info: ProjectConfig) -> Self {
    Self {
      name: info.server_name.into_boxed_str(),
      version: info.mc_version.0.into_boxed_str(),
      flavor: info.flavor_version.into_boxed_str(),
    }
  }
}
