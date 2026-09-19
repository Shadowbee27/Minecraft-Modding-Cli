use color_eyre::Result;
use colorful::{Color, Colorful};
use inquire::InquireError;
use rayon::prelude::*;
use std::process::exit;

use crate::software::{McVersion, ServerSoftware, SupportedVersion};
use crate::tui::common::simple_prompt::{single_choice, single_choice_fallback};

#[derive(Debug, Clone)]
enum McVersionQuestion {
  ShowUnstable(bool),
  Version(SupportedVersion),
}

impl std::fmt::Display for McVersionQuestion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      McVersionQuestion::ShowUnstable(s) => {
        if *s {
          write!(f, "<Show snapshots>")
        } else {
          write!(f, "<Hide snapshot>")
        }
      }
      McVersionQuestion::Version(v) => {
        write!(f, "{}", v.version)
      }
    }
  }
}

// This is prop spaghetti code
impl McVersionQuestion {
  pub fn from_supported_version(
    supported_version: Vec<SupportedVersion>,
    show_unstable: bool,
  ) -> Vec<McVersionQuestion> {
    let mut options = Vec::new();
    options.push(McVersionQuestion::ShowUnstable(show_unstable));
    for v in supported_version {
      if show_unstable && v.stable || !show_unstable {
        options.push(McVersionQuestion::Version(v));
      }
    }
    options
  }
}

pub fn mc_version_select(software: &ServerSoftware) -> Result<McVersion> {
  let mut show_unstable = true;
  let mc_versions = ServerSoftware::get_supported_minecraft_versions(software)?;
  let mut question = McVersionQuestion::from_supported_version(mc_versions.clone(), show_unstable);
  loop {
    match single_choice(
      question,
      "Which Minecraft version do you want to use?",
      "Esc to cancel",
    ) {
      Ok(ans) => match ans {
        McVersionQuestion::Version(v) => return color_eyre::eyre::Ok(McVersion(v.version)),
        McVersionQuestion::ShowUnstable(_) => {
          show_unstable = !show_unstable;
          question = McVersionQuestion::from_supported_version(mc_versions.clone(), show_unstable);
        }
      },
      Err(e) => match e {
        InquireError::NotTTY => {
          eprintln!(
            "Terminal is not functional for TUI : ({})",
            e.to_string().color(Color::Red).bold()
          );
          break;
        }
        InquireError::Custom(e) => {
          eprintln!(
            "Unknown Error: ({})",
            e.to_string().color(Color::Red).bold()
          );
          break;
        }
        InquireError::IO(e) => {
          eprintln!(
            "IO Error: ({}). Using Fallback",
            e.to_string().color(Color::Red).bold()
          );
          break;
        }
        InquireError::InvalidConfiguration(_) => unreachable!(),
        InquireError::OperationCanceled | InquireError::OperationInterrupted => {
          eprintln!("Operation canceled by user");
          exit(1)
        }
      },
    }
  }

  color_eyre::eyre::Ok(McVersion(single_choice_fallback(
    mc_versions.par_iter().map(|v| v.version.clone()).collect(),
    "Which Minecraft version do you want to use?",
    "Leave empty to cancel",
  )?))
}
