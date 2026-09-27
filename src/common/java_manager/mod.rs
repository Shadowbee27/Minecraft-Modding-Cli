use color_eyre::{
  Result,
  eyre::{Context, ContextCompat, eyre},
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JavaVersion {
  JAVA8(Java8),
  JAVA17(Java17),
  JAVA25(Java25),
  JAVA21(Java21),
}

impl JavaVersion {
  pub fn from_java_info(java_info: &JavaInfo) -> Result<JavaVersion> {
    let java_ver: JavaVersion = match java_info
      .java_runtime_version
      .split_once(".")
      .wrap_err("Invalid Java version {java_info.java_runtime_version}")?
      .0
      .parse::<i32>()
      .wrap_err("Invalid Java version {java_info.java_runtime_version}")?
    {
      8 => JavaVersion::JAVA8(Java8 {
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
