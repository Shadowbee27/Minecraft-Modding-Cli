use color_eyre::eyre::Result;
use colorful::{Color, Colorful};
use inquire::InquireError;
use std::{fmt, process::exit};

use crate::tui::common::simple_prompt::{single_choice, single_choice_fallback};

// General version Question
pub fn select_flavor_version<T>(versions: Vec<T>) -> Result<T>
where
  T: Clone + fmt::Display,
{
  match single_choice(
    versions.clone(),
    "Which Server software version do you want to use?",
    "Esc to cancel",
  ) {
    Ok(ans) => {
      return Ok(ans);
    }
    Err(e) => match e {
      InquireError::NotTTY => {
        eprintln!(
          "Terminal is not functional for TUI : ({})",
          e.to_string().color(Color::Red).bold()
        );
      }
      InquireError::Custom(e) => {
        eprintln!(
          "Unknown Error: ({})",
          e.to_string().color(Color::Red).bold()
        );
      }
      InquireError::IO(e) => {
        eprintln!(
          "IO Error: ({}). Using Fallback",
          e.to_string().color(Color::Red).bold()
        );
      }
      InquireError::InvalidConfiguration(_) => unreachable!(),
      InquireError::OperationCanceled | InquireError::OperationInterrupted => {
        eprintln!("Operation canceled by user");
        exit(1)
      }
    },
  }
  single_choice_fallback(
    versions,
    "Which Server software version do you want to use?",
    "Leave empty to cancel",
  )
}

// Special versions e.g. Fabric installer

pub fn select_fabric_installer_version(installer_versions: Vec<&str>) -> Result<&str> {
  match single_choice(
    installer_versions.clone(),
    "Which Fabric Installer do you want to use?",
    "Esc to cancel",
  ) {
    Ok(ans) => {
      return color_eyre::eyre::Ok(ans);
    }
    Err(e) => match e {
      InquireError::NotTTY => {
        eprintln!(
          "Terminal is not functional for TUI : ({})",
          e.to_string().color(Color::Red).bold()
        );
      }
      InquireError::Custom(e) => {
        eprintln!(
          "Unknown Error: ({})",
          e.to_string().color(Color::Red).bold()
        );
      }
      InquireError::IO(e) => {
        eprintln!(
          "IO Error: ({}). Using Fallback",
          e.to_string().color(Color::Red).bold()
        );
      }
      InquireError::InvalidConfiguration(_) => unreachable!(),
      InquireError::OperationCanceled | InquireError::OperationInterrupted => {
        eprintln!("Operation canceled by user");
        exit(1)
      }
    },
  }
  color_eyre::eyre::Ok(single_choice_fallback(
    installer_versions,
    "Which Fabric Installer do you want to use?",
    "Leave empty to cancel",
  )?)
}
