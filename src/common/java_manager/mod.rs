use std::{fmt, str::FromStr};

use color_eyre::{
  Result,
  eyre::{Context, ContextCompat, eyre},
};
use date::Date;

use crate::{
  common::java_manager::find::find_javas, software::McVersion,
  tui::dialogs::java::select_java_version_for_mc_version,
};

use super::java_manager::{
  check::JavaInfo, eight::Java8, seventeen::Java17, twenty_five::Java25, twenty_one::Java21,
};

pub mod check;
pub mod eight;
pub mod find;
pub mod java_paths;
pub mod seventeen;
pub mod twenty_five;
pub mod twenty_one;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JavaVersion {
  JAVA8(Java8),
  JAVA17(Java17),
  JAVA25(Java25),
  JAVA21(Java21),
}

impl fmt::Display for JavaVersion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      JavaVersion::JAVA8(java) => write!(
        f,
        "Java: {}\t| By: {}\t| At: {}\t| For: {}",
        java.version,
        java.vendor,
        java.path.to_str().unwrap(),
        java.arch
      ),
      JavaVersion::JAVA17(java) => write!(
        f,
        "Java: {}\t| By: {}\t| At: {}\t| For: {}",
        java.version,
        java.vendor,
        java.path.to_str().unwrap(),
        java.arch
      ),
      JavaVersion::JAVA25(java) => write!(
        f,
        "Java: {}\t| By: {}\t| At: {}\t| For: {}",
        java.version,
        java.vendor,
        java.path.to_str().unwrap(),
        java.arch
      ),
      JavaVersion::JAVA21(java) => write!(
        f,
        "Java: {}\t| By: {}\t| At: {}\t| For: {}",
        java.version,
        java.vendor,
        java.path.to_str().unwrap(),
        java.arch
      ),
    }
  }
}

impl JavaVersion {
  pub fn is_stable(&self, mc_version: &McVersion) -> bool {
    return if &mc_version.1 < Date::from_str("2021-05-12").unwrap() {
      // Java 8
      self.is_java8()
    } else if &mc_version.1 < Date::from_str("2021-09-15").unwrap() {
      // Java 16
      self.is_java17()
    } else if &mc_version.1 < Date::from_str("2024-04-03").unwrap() {
      // Java 17
      self.is_java17()
    } else if &mc_version.1 < Date::from_str("2025-12-16").unwrap() {
      // Java 21
      self.is_java21()
    } else {
      // Java 25
      self.is_java25()
    };
  }

  pub fn is_java8(&self) -> bool {
    match self {
      Self::JAVA8(_) => true,
      _ => false,
    }
  }

  pub fn is_java17(&self) -> bool {
    match self {
      Self::JAVA17(_) => true,
      _ => false,
    }
  }

  pub fn is_java21(&self) -> bool {
    match self {
      Self::JAVA21(_) => true,
      _ => false,
    }
  }

  pub fn is_java25(&self) -> bool {
    match self {
      Self::JAVA25(_) => true,
      _ => false,
    }
  }

  pub fn from_java_info(java_info: &JavaInfo) -> Result<JavaVersion> {
    dbg!(
      java_info
        .java_runtime_version
        .split_once(".")
        .wrap_err("Invalid Java version {java_info.java_runtime_version}")?
        .0
        .parse::<i32>()
        .unwrap()
    );

    let java_ver: JavaVersion = match java_info
      .java_runtime_version
      .split_once(".")
      .wrap_err("Invalid Java version {java_info.java_runtime_version}")?
      .0
      .parse::<i32>()
      .wrap_err("Invalid Java version {java_info.java_runtime_version}")?
    {
      // Example 1.8.0_504-b01
      1 | 8 => JavaVersion::JAVA8(Java8 {
        version: java_info.java_runtime_version.to_owned(),
        vendor: java_info.java_vendor.to_owned(),
        arch: java_info.os_arch.to_owned(),
        path: java_info.path.to_owned(),
      }),
      17 => JavaVersion::JAVA17(Java17 {
        version: java_info.java_runtime_version.to_owned(),
        vendor: java_info.java_vendor.to_owned(),
        arch: java_info.os_arch.to_owned(),
        path: java_info.path.to_owned(),
      }),
      21 => JavaVersion::JAVA21(Java21 {
        version: java_info.java_runtime_version.to_owned(),
        vendor: java_info.java_vendor.to_owned(),
        arch: java_info.os_arch.to_owned(),
        path: java_info.path.to_owned(),
      }),
      25 => JavaVersion::JAVA25(Java25 {
        version: java_info.java_runtime_version.to_owned(),
        vendor: java_info.java_vendor.to_owned(),
        arch: java_info.os_arch.to_owned(),
        path: java_info.path.to_owned(),
      }),
      _ => return Err(eyre!("Unknown Java Version")),
    };

    Ok(java_ver)
  }
}

pub trait Java {
  fn install() -> Result<()>;
}

pub fn select_java_version(mc_version: &McVersion) -> Result<JavaVersion> {
  select_java_version_for_mc_version(find_javas()?, mc_version)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_stable() {
    dbg!(JavaVersion::is_stable(
      &JavaVersion::JAVA25(Java25::default()),
      &McVersion("26.3".to_string(), Date::today_utc()),
    ));
  }

  #[test]
  fn select_java() {
    dbg!(select_java_version(&McVersion("26.3".to_string(), Date::today_utc())).unwrap());
  }
}
