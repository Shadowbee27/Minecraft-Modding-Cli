use clap::*;
use color_eyre::Result;

use crate::command::{completions, init, java, new, run, update};

pub mod command;
pub mod common;
pub mod config;
pub mod mod_source;
pub mod software;
pub mod tui;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[command(subcommand)]
  action: BaseAction,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum BaseAction {
  /// Run a server
  Run(run::RunArgs),
  /// Initialize a existing directory
  Init(init::InitArgs),
  /// Update a server
  Update(update::UpdateArgs),
  /// Create a new server
  New(new::CreateNewArgs),
  #[command(subcommand)]
  /// Manage java
  Java(java::JavaManagerArgs), // TODO: Update(software & mods), Install, ...
  /// Generate autocompletion script
  Complete(completions::CompletionArgs),
}

#[macro_export]
macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
}

fn main() -> Result<()> {
  color_eyre::install()?;
  better_panic::install();

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
    BaseAction::Java(javamanagerargs) => java::java_manager(javamanagerargs)?,
    BaseAction::Complete(completionargs) => completions::completions(completionargs),
    BaseAction::Update(update_args) => update::update(update_args)?,
  }
  Ok(())
}
