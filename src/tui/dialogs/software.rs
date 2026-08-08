use colorful::{Color, Colorful};
use inquire::InquireError;
use std::process::exit;
use strum::IntoEnumIterator;

use crate::software::ServerSoftware;
use crate::tui::common::simple_prompt::{single_choice, single_choice_fallback};

pub fn server_software_selector() -> ServerSoftware {
  loop {
    match single_choice(
      ServerSoftware::iter().collect(),
      "Which Server software do you want to use?",
      "Esc to cancel",
    ) {
      Ok(ans) => {
        return ans;
      }
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
  single_choice_fallback(
    ServerSoftware::iter().collect(),
    "Which Server software do you want to use?",
    "Leave empty to cancel",
  )
}
