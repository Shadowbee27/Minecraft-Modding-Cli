use color_eyre::{Result, eyre::eyre};
use colorful::{Color, Colorful};
use inquire::InquireError;
use strum::IntoEnumIterator;

use crate::{
  mod_source::ModSource,
  tui::common::simple_prompt::{single_choice, single_choice_fallback},
};

pub fn preferred_mod_source() -> Result<ModSource> {
  match single_choice(
    ModSource::iter().collect(),
    "Which Mod-Source should be prioritized?",
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
        return Err(eyre!("Can't proceed without the prioriticed Modsource"));
      }
    },
  }
  single_choice_fallback(
    ModSource::iter().collect(),
    "Which Mod-Source should be prioritized?",
    "Leave empty to cancel",
  )
}
