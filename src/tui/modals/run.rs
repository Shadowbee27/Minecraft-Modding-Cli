use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExeArgs {
  pub name: String,
  pub jar: String,
  pub ram: Ram,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ram {
  pub xmx: MemorySize,
  pub xms: MemorySize,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MemorySize {
  pub amount: f64,
  pub unit: MemoryUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryUnit {
  M,
  G,
}

impl ExeArgs {
  pub fn run(&self) -> Result<()> {
    unimplemented!()
  }
}

impl fmt::Display for Ram {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "-Xms{} -Xmx{}", self.xms, self.xmx)
  }
}

impl fmt::Display for MemorySize {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.amount, self.unit)
  }
}

impl fmt::Display for MemoryUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MemoryUnit::M => write!(f, "M"),
      MemoryUnit::G => write!(f, "G"),
    }
  }
}
impl MemorySize {
  pub fn switch(self) -> Self {
    match &self.unit {
      MemoryUnit::G => self.to_megabytes(),
      MemoryUnit::M => self.to_gigabytes(),
    }
  }

  pub fn to_gigabytes(mut self) -> Self {
    self.amount /= 1024.0;
    self
  }

  pub fn to_megabytes(mut self) -> Self {
    self.amount *= 1024.0;
    self
  }
}

pub fn init_run_server_modal(_exe_args: ExeArgs) -> Result<()> {
  ratatui::run(app)?;
  Ok(())
}

pub struct RunApp {}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
  loop {
    terminal.draw(render)?;
    if crossterm::event::read()?.is_key_press() {
      break Ok(());
    }
  }
}

fn render(frame: &mut Frame) {
  frame.render_widget("hello world", frame.area());
}
