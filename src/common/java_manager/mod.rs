use color_eyre::Result;

use crate::common::java_manager::{
  eight::JavaEight, seventeen::JavaSeventeen, twenty_five::JavaTwentyFive,
  twenty_one::JavaTwentyOne,
};

pub mod check;
pub mod eight;
pub mod seventeen;
pub mod twenty_five;
pub mod twenty_one;

#[derive(Debug, Clone)]
pub enum JavaVersion {
  EIGHT(JavaEight),
  SEVENTEEN(JavaSeventeen),
  TWENTYFIVE(JavaTwentyFive),
  TWENTYONE(JavaTwentyOne),
}

pub trait Java {
  fn install() -> Result<()>;
}
