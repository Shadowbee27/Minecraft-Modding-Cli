use std::{fmt, process::exit};

use color_eyre::eyre::Result;
use colorful::{Color, Colorful};
use inquire::InquireError;

use crate::{
  software::ServerSoftware,
  tui::{
    common::simple_prompt::{single_choice, single_choice_fallback},
    dialogs::{EMPTY_TO_CANCEL, ESC_TO_CANCEL},
  },
};

#[derive(Clone, Debug)]
pub enum WhatToUpdate<'a> {
  MINECRAFT,
  FLAVOR(&'a ServerSoftware),
}

impl<'a> fmt::Display for WhatToUpdate<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      WhatToUpdate::MINECRAFT => write!(f, "Minecraft"),
      WhatToUpdate::FLAVOR(server_software) => {
        write!(f, "{}", server_software)
      }
    }
  }
}

pub fn what_to_update<'a>(flavor: &'a ServerSoftware) -> Result<WhatToUpdate<'a>> {
  let what_to_update = match flavor {
    ServerSoftware::VANILLA(_) => vec![WhatToUpdate::MINECRAFT],
    _ => vec![WhatToUpdate::MINECRAFT, WhatToUpdate::FLAVOR(flavor)],
  };

  match single_choice(
    what_to_update.clone(),
    "What do you want to update?",
    ESC_TO_CANCEL,
  ) {
    Ok(c) => return Ok(c),
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

  single_choice_fallback(what_to_update, "", EMPTY_TO_CANCEL)
}
