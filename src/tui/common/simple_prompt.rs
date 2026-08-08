use std::{io, process::exit};

use inquire::{InquireError, Select};

pub fn single_choice<T: std::fmt::Display>(
  list: Vec<T>,
  question: &str,
  help_msg: &str,
) -> Result<T, InquireError> {
  Ok(
    Select::new(question, list)
      .with_help_message(help_msg)
      .prompt()?,
  )
}

pub fn single_choice_fallback<T: std::fmt::Display + Clone>(
  list: Vec<T>,
  question: &str,
  help_msg: &str,
) -> T {
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
      println!("Action canceled by user");
      exit(1)
    }
    for option in &list {
      if answ == option.to_string().to_lowercase() {
        return option.clone();
      }
    }
    eprintln!("Unknown option");
  }
}
