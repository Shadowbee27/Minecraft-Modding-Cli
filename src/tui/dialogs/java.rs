use crate::{
  common::java_manager::JavaVersion,
  software::McVersion,
  tui::{
    common::simple_prompt::{single_choice, single_choice_fallback},
    dialogs::{EMPTY_TO_CANCEL, ESC_TO_CANCEL},
  },
};
use color_eyre::Result;
use colorful::{Color, Colorful};
use inquire::InquireError;
use std::{fmt, process::exit};
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum JavaSelect<'a> {
  Java(&'a JavaVersion),
  ShowUnstable(bool),
}

impl<'a> JavaSelect<'a> {
  fn from_java_version(
    javas: &'a Vec<JavaVersion>,
    show_unstable: bool,
    mc_version: &McVersion,
  ) -> Vec<JavaSelect<'a>> {
    let mut java_select = Vec::new();

    for j in javas {
      if show_unstable || j.is_stable(mc_version).unwrap() {
        java_select.push(JavaSelect::Java(j));
      }
    }
    java_select.push(JavaSelect::ShowUnstable(show_unstable));
    java_select
  }
}

impl<'a> fmt::Display for JavaSelect<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      JavaSelect::Java(j) => {
        write!(f, "{}", j)
      }
      JavaSelect::ShowUnstable(b) => {
        if *b {
          write!(f, "<Hide unsuppored javas>")
        } else {
          write!(f, "<Show unsuppored javas>")
        }
      }
    }
  }
}

pub fn select_java_version_for_mc_version(
  javas: Vec<JavaVersion>,
  mc_version: &McVersion,
) -> Result<JavaVersion> {
  let mut show_unstable = false;

  let mut question = JavaSelect::from_java_version(&javas, show_unstable, mc_version);
  loop {
    match single_choice(
      question,
      "Which Java install do you want to use?",
      ESC_TO_CANCEL,
    ) {
      Ok(ans) => match ans {
        JavaSelect::Java(j) => return color_eyre::eyre::Ok(j.clone()),
        JavaSelect::ShowUnstable(_) => {
          show_unstable = !show_unstable;
          question = JavaSelect::from_java_version(&javas, show_unstable, mc_version);
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

  single_choice_fallback(
    javas,
    "Which Java install do you want to use?",
    EMPTY_TO_CANCEL,
  )
}

pub fn select_java_version() -> Result<JavaVersion> {
  match single_choice(
    JavaVersion::iter().map(|j| j.print()).collect(),
    "Which Java version do you want to install?",
    ESC_TO_CANCEL,
  ) {
    Ok(ans) => return JavaVersion::form_string(&ans),
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
    JavaVersion::iter().collect(),
    "Which Java version do you want to install?",
    EMPTY_TO_CANCEL,
  )
}

pub fn select_java(javas: Vec<JavaVersion>) -> Result<JavaVersion> {
  match single_choice(
    javas.clone(),
    "Which Java install do you want to use?",
    ESC_TO_CANCEL,
  ) {
    Ok(ans) => return Ok(ans),
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
    javas,
    "Which Java install do you want to use?",
    EMPTY_TO_CANCEL,
  )
}
