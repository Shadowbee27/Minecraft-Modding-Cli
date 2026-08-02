use clap::*;

pub mod dialogs;
use crate::command::{init, new};
pub mod command;
pub mod software;

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
  let args = Args::parse();
  match args.action {
    BaseAction::Init(initargs) => init::init_dir(initargs),
    BaseAction::New(newargs) => new::create_new_server(newargs),
  }
}
