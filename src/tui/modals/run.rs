use color_eyre::Result;

#[derive(Debug, Clone)]
pub struct ExeArgs {
  pub name: String,
  pub jar: String,
  pub ram: Ram,
}

#[derive(Debug, Clone, Copy)]
pub struct Ram {
  pub xmx: MemorySize,
  pub xms: MemorySize,
}

#[derive(Debug, Clone, Copy)]
pub struct MemorySize {
  pub amount: u128,
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

impl MemorySize {
  pub fn switch(self) -> Self {
    match &self.unit {
      MemoryUnit::G => self.to_megabytes(),
      MemoryUnit::M => self.to_gigabytes(),
    }
  }

  pub fn to_gigabytes(mut self) -> Self {
    self.amount /= 1024;
    self
  }

  pub fn to_megabytes(mut self) -> Self {
    self.amount *= 1024;
    self
  }
}

pub fn init_run_server_modal(_exe_args: ExeArgs) -> Result<()> {
  unimplemented!()
}
