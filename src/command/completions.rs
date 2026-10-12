use std::io::stdout;

use clap::{CommandFactory, clap_derive::*};
use clap_complete::aot::{Shell, generate};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Parser)]
pub struct CompletionArgs {
  /// Shell for autocompletion script for bash, elvish, fish, powershell, zsh
  pub shell: Shell,
}

pub fn completions(args: CompletionArgs) {
  generate(
    args.shell,
    &mut crate::Args::command(),
    "mcmc",
    &mut stdout(),
  );
}
