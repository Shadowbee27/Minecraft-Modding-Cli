use semver::Version;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path};
use strum::EnumIter;

use crate::{
  config::ProjectConfig,
  software::{
    bukkit::Bukkit, fabric::Fabric, folia::Folia, forge::Forge, legacy_fabric::LegacyFabric,
    neoforge::NeoForge, paper::Paper, purpur::Purpur, quilt::Quilt, spiggot::Spiggot,
    vanilla::Vanilla,
  },
};

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

// Unlikely to be supported: Sponge, Babric, BTA (Babric), Java agent, liteloader RisugamisModLoader, Nil loader, Ornithe, Rift
// Todo: bukkit, fabric, folia, forge, Legacy Fabric, neoforge, paper, purpur, quilt, spiggot, vanilla
// install done: fabric
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportedVersion {
  pub version: String, // semver can't be used thx to minecraft
  pub stable: bool,
}

impl std::fmt::Display for SupportedVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.version)
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McVersion(pub String);

pub trait SeverSoftware {
  fn update(&mut self);
  fn install(&mut self, pconf: ProjectConfig, pdir: &Path);
  fn get_supported_minecraft_versions() -> Vec<SupportedVersion>;
}

impl ServerSoftware {
  pub fn get_supported_minecraft_versions(&self) -> Vec<SupportedVersion> {
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
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Self::FABRIC(_) => "Fabric",
      Self::FOLIA(_) => "Folia",
      Self::FORGE(_) => "Forge",
      Self::NEOFORGE(_) => "Neoforge",
      Self::PAPER(_) => "Paper",
      Self::VANILLA(_) => "Vanilla",
      Self::BUKKIT(_) => "Bukkit",
      Self::QUILT(_) => "Quilt",
      Self::LegacyFabric(_) => "Legacy Fabric",
      Self::PURPUR(_) => "Purpur",
      Self::SPIGGOT(_) => "Spiggot",
    }
  }
  pub fn install(&mut self, project_config: ProjectConfig, server_dir: &Path) {
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
    }
  }

  pub fn get_moddification_dir(&self) -> Option<&str> {
    let dir: Option<&str> = match self {
      Self::FABRIC(_)
      | Self::FORGE(_)
      | Self::NEOFORGE(_)
      | Self::QUILT(_)
      | Self::LegacyFabric(_) => Some("mods/"),
      Self::FOLIA(_) | Self::PAPER(_) | Self::BUKKIT(_) | Self::SPIGGOT(_) | Self::PURPUR(_) => {
        Some("plugins/")
      }
      Self::VANILLA(_) => None,
    };
    dir
  }

  pub fn get_flavor_version(self) -> Version {
    match self {
      Self::FABRIC(f) => f.flavor_version,
      Self::FOLIA(f) => f.flavor_version,
      Self::FORGE(f) => f.flavor_version,
      Self::NEOFORGE(n) => n.flavor_version,
      Self::PAPER(p) => p.flavor_version,
      Self::BUKKIT(b) => b.flavor_version,
      Self::QUILT(q) => q.flavor_version,
      Self::VANILLA(v) => v.flavor_version,
      Self::LegacyFabric(l) => l.flavor_version,
      Self::PURPUR(p) => p.flavor_version,
      Self::SPIGGOT(s) => s.flavor_version,
    }
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
    }
  }
}
