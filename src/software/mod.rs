use std::path::Path;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{
  config::ProjectConfig,
  software::{
    fabric::Fabric, folia::Folia, forge::Forge, neoforge::NeoForge, paper::Paper, vanilla::Vanilla,
  },
};

pub mod bukkit;
pub mod fabric;
pub mod folia;
pub mod forge;
pub mod neoforge;
pub mod paper;
pub mod quilt;
pub mod vanilla;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ServerSoftware {
  FABRIC(Fabric),
  FOLIA(Folia),
  FORGE(Forge),
  NEOFORGE(NeoForge),
  PAPER(Paper),
  VANILLA(Vanilla),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McVersion(pub Version);

// pub struct SoftwareUpdateError {}
// pub struct SoftwareInstallError {}

pub trait SeverSoftware {
  fn update(&self);
  fn install(&self, pconf: ProjectConfig, pdir: &Path);
}

impl ServerSoftware {
  pub fn get_moddification_dir(&self) -> Option<&str> {
    let dir: Option<&str> = match self {
      Self::FABRIC(_) | Self::FORGE(_) | Self::NEOFORGE(_) => Some("mods/"),
      Self::FOLIA(_) | Self::PAPER(_) => Some("plugins/"),
      Self::VANILLA(_) => None,
    };
    dir
  }

  pub fn get_flavor_version(self) -> Version {
    match self {
      ServerSoftware::FABRIC(f) => f.flavor_version,
      ServerSoftware::FOLIA(f) => f.flavor_version,
      ServerSoftware::FORGE(f) => f.flavor_version,
      ServerSoftware::NEOFORGE(n) => n.flavor_version,
      ServerSoftware::PAPER(p) => p.flavor_version,
      ServerSoftware::VANILLA(v) => v.flavor_version,
    }
  }
}
