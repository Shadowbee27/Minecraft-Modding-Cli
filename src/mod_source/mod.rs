use std::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub mod curseforge;
pub mod index;
pub mod modrinth;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModSource {
  // CURSEFORGE,
  MODRINTH,
}

impl fmt::Display for ModSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      //  Self::CURSEFORGE => write!(f, "Curseforge"),
      Self::MODRINTH => write!(f, "Modrinth"),
    }
  }
}
