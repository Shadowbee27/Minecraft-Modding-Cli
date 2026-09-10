use clap::*;
use color_eyre::Result;

use crate::command::{init, new, run};

pub mod command;
pub mod common;
pub mod config;
pub mod mod_source;
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
  Run(run::RunArgs),
  Init(init::InitArgs),
  New(new::CreateNewArgs),
  // TODO: Update(software & mods), Install, ...
}

fn main() -> Result<()> {
  color_eyre::install()?;

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
    BaseAction::Init(initargs) => init::init_dir(initargs)?,
    BaseAction::New(newargs) => new::create_new_server(newargs)?,
    BaseAction::Run(runargs) => run::run_server(runargs)?,
  }
  Ok(())
}
