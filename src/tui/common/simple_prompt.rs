use color_eyre::{Result, eyre::eyre};
use colorful::{Color, Colorful};
use inquire::{InquireError, Select};
use std::{fmt, io, process::exit};
use strum::{EnumIter, IntoEnumIterator};

use crate::tui::dialogs::ESC_TO_CANCEL;

pub fn single_choice<T: std::fmt::Display>(
  list: Vec<T>,
  question: &str,
  help_msg: &str,
) -> Result<T, InquireError> {
  Select::new(question, list)
    .with_help_message(help_msg)
    .prompt()
}

pub fn single_choice_fallback<T: std::fmt::Display + Clone>(
  list: Vec<T>,
  question: &str,
  help_msg: &str,
) -> Result<T> {
  loop {
    println!("{}", question);
    for option in &list {
      println!("{}", option);
    }
    println!("[{}]", help_msg);
    let mut answ = String::new();
    io::stdin()
      .read_line(&mut answ)
      .expect("Failed to read answer");

    answ = answ.trim().to_lowercase();
    if answ.is_empty() {
      return Err(eyre!("Action canceled by user"));
    }

    for option in &list {
      if answ == option.to_string().to_lowercase() {
        return Ok(option.clone());
      }
    }
    eprintln!("Unknown option");
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
enum YesNO {
  Yes,
  No,
}

impl fmt::Display for YesNO {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      YesNO::Yes => write!(f, "Yes"),
      YesNO::No => write!(f, "No"),
    }
  }
}

pub fn yes_no_prompt(question: &str, help_msg: Option<&str>) -> bool {
  match Select::new(question, YesNO::iter().collect())
    .with_help_message(help_msg.unwrap_or(ESC_TO_CANCEL))
    .prompt()
  {
    Ok(r) => match r {
      YesNO::Yes => return true,
      YesNO::No => return false,
    },
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

  false
}
