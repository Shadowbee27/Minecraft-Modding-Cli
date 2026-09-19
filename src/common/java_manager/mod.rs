use color_eyre::Result;

use crate::common::java_manager::{
  eight::Java8, seventeen::Java17, twenty_five::Java25, twenty_one::Java21,
};

pub mod check;
pub mod eight;
pub mod find;
pub mod java_paths;
pub mod seventeen;
pub mod twenty_five;
pub mod twenty_one;

#[derive(Debug, Clone)]
pub enum JavaVersion {
  JAVA8(Java8),
  JAVA17(Java17),
  JAVA25(Java25),
  JAVA21(Java21),
}

pub trait Java {
  fn install() -> Result<()>;
}
