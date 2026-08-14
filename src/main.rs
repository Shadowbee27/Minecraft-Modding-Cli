use clap::*;
use colorful::{Color, Colorful};
use std::fs::{exists, read_to_string};
use std::process::exit;
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;

use crate::command::{init, new};
use crate::config::{CONFIG_PATH, ProjectConfig};

pub mod command;
pub mod common;
pub mod config;
pub mod software;
pub mod tui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[command(subcommand)]
  action: BaseAction,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum BaseAction {
  Init(init::InitArgs),
  New(new::CreateNewArgs),
  // TODO: Update(software & mods), Install, ...
}

fn main() {
  // // a builder for `FmtSubscriber`.
  // let subscriber = FmtSubscriber::builder()
  //   // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
  //   // will be written to stdout.
  //   .with_max_level(Level::WARN)
  //   // completes the builder.
  //   .finish();

  // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let args = Args::parse();
  match args.action {
    BaseAction::Init(initargs) => init::init_dir(initargs),
    BaseAction::New(newargs) => new::create_new_server(newargs),
  }
}

fn load_config() -> ProjectConfig {
  let exists = match exists(CONFIG_PATH) {
    Ok(bool) => bool,
    Err(e) => {
      eprintln!(
        "Failed to check if config exists: ({})!",
        e.to_string().color(Color::Red).bold()
      );
      exit(1)
    }
  };

  if !exists {
    eprintln!("Config not found in current directory!");
    exit(1)
  }

  let config = match read_to_string(CONFIG_PATH) {
    Ok(f) => f,
    Err(e) => {
      eprintln!(
        "Failed to read config: ({})",
        e.to_string().color(Color::Red).bold()
      );
      exit(1)
    }
  };

  let toml_config: ProjectConfig = match toml::from_str(&config) {
    Ok(c) => c,
    Err(e) => {
      eprintln!(
        "Parsing config failed:\n {}",
        e.to_string().color(Color::Red).bold()
      );
      exit(1)
    }
  };

  toml_config
}
