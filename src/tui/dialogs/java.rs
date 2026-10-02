use std::{fmt, process::exit};

use crate::{
  common::java_manager::JavaVersion,
  software::McVersion,
  tui::common::simple_prompt::{single_choice, single_choice_fallback},
};
use color_eyre::Result;
use colorful::{Color, Colorful};
use inquire::InquireError;

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
      if show_unstable {
        java_select.push(JavaSelect::Java(j));
      } else if j.is_stable(mc_version) {
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

  let mut question = JavaSelect::from_java_version(&javas, show_unstable, &mc_version);
  loop {
    match single_choice(
      question,
      "Which Java install do you want to use?",
      "Esc to cancel",
    ) {
      Ok(ans) => match ans {
        JavaSelect::Java(j) => return color_eyre::eyre::Ok(j.clone()),
        JavaSelect::ShowUnstable(_) => {
          show_unstable = !show_unstable;
          question = JavaSelect::from_java_version(&javas, show_unstable, &mc_version);
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
    "Which Minecraft version do you want to use?",
    "Leave empty to cancel",
  )
}
