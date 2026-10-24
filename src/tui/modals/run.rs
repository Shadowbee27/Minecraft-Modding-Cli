use color_eyre::{Result, eyre::Context};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::Rect,
  style::Stylize,
  symbols::border,
  text::{Line, Text},
  widgets::{Block, Paragraph, Widget, Wrap},
};
use serde::{Deserialize, Serialize};
use std::{
  fmt,
  io::{self, BufReader},
  path::PathBuf,
  process::{Child, Command, Stdio},
  sync::mpsc::{self, Receiver, Sender},
  thread,
};
use std::{
  io::{BufRead, Read},
  time::Duration,
};

use crate::{
  common::java_manager::JavaVersion,
  software::{McVersion, ServerDetails},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExeArgs {
  pub path: PathBuf,
  pub mc_ver: McVersion,
  pub java: Option<JavaVersion>,
  pub name: String,
  pub jar: String,
  pub ram: Ram,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ram {
  pub xmx: MemorySize,
  pub xms: MemorySize,
}

impl Default for Ram {
  fn default() -> Self {
    Ram {
      xms: MemorySize {
        amount: 512.0,
        unit: MemoryUnit::M,
      },
      xmx: MemorySize {
        amount: 2048.0,
        unit: MemoryUnit::M,
      },
    }
  }
}

impl Ram {
  fn to_string(&self) -> String {
    format!("{self}")
  }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MemorySize {
  pub amount: f64,
  pub unit: MemoryUnit,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryUnit {
  M,
  G,
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

pub fn init_run_server_modal(exe_args: ExeArgs, server_info: ServerDetails<'_>) -> Result<()> {
  ratatui::run(|terminal| RunApp::new(exe_args, server_info).run(terminal))?;
  Ok(())
}

#[derive(Debug)]
pub struct RunApp<'a> {
  pub auto_scroll: bool,
  pub scroll: u16,
  pub logs: Vec<Line<'a>>,
  pub log_rx: Receiver<String>,
  pub log_tx: Sender<String>,
  pub exe_args: ExeArgs,
  pub server: Option<Child>,
  pub server_info: ServerDetails<'a>,
  pub exit: bool,
  pub state: RunState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunState {
  Exiting,  // Popup asking the user if the server should be stopped
  Console,  // the user is able to input commands into to the server console
  Overview, // Just showing the logs with the option to exit or enter console mode
}

impl<'a> RunApp<'a> {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    self.start_process(self.exe_args.clone());
    while !self.exit {
      if event::poll(Duration::from_millis(50))? {
        self.update_logs();
      }

      terminal.draw(|frame| self.draw(frame))?;

      if event::poll(Duration::from_millis(50))? {
        self.handle_events()?;
      }
    }
    Ok(())
  }

  fn draw(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      // it's important to check that the event is a key press event as
      // crossterm also emits key release and repeat events on Windows.
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)
      }
      _ => {}
    };
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    if self.state == RunState::Console {
      match key_event.code {
        KeyCode::Esc => self.state = RunState::Overview,
        _ => {}
      }
    } else if self.state == RunState::Overview {
      match key_event.code {
        KeyCode::Char('q') => self.state = RunState::Exiting,
        KeyCode::Enter => self.state = RunState::Console,
        KeyCode::Up => {
          self.auto_scroll = false;
          self.scroll = self.scroll.saturating_sub(1);
        }
        KeyCode::Down => {
          self.scroll = self.scroll.saturating_add(1);
        }
        KeyCode::End => {
          self.auto_scroll = true;
        }
        _ => {}
      }
    } else if self.state == RunState::Exiting {
      match key_event.code {
        // TODO: Find a good key to not press accedentily
        KeyCode::Char('q') | KeyCode::Esc => self.state = RunState::Overview,
        KeyCode::Char('y') => self.exit = true,
        _ => {}
      }
    }
  }

  pub fn start_process(&mut self, exe_args: ExeArgs) {
    let mut child = Command::new(
      exe_args
        .java
        .clone()
        .unwrap_or_else(|| {
          McVersion::get_default_java(&exe_args.mc_ver)
            .wrap_err("Failed to get default java version")
            .unwrap()
        })
        .get_path(),
    )
    .args(exe_args.ram.to_string().split_whitespace())
    .args(["-jar", &exe_args.jar.clone(), "--nogui"])
    .current_dir(exe_args.path)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    Self::read_logs(stdout, "stdout", self.log_tx.clone());
    Self::read_logs(stderr, "stderr", self.log_tx.clone());
  }

  pub fn update_logs(&mut self) {
    while let Ok(line) = self.log_rx.recv_timeout(Duration::from_millis(25)) {
      self.logs.push(Line::from(line));
    }
  }

  fn read_logs<R: Read + Send + 'static>(stream: R, name: &'static str, tx: Sender<String>) {
    thread::spawn(move || {
      for line in BufReader::new(stream).lines() {
        match line {
          Ok(line) => {
            let _ = tx.send(format!("[{name}] {line}"));
          }
          Err(_) => break,
        }
      }
    });
  }

  fn new(exe_args: ExeArgs, server_info: ServerDetails<'a>) -> Self {
    let (log_tx, log_rx) = mpsc::channel();
    RunApp {
      auto_scroll: true,
      scroll: 0,
      server: None,
      exe_args,
      logs: vec![],
      log_rx,
      log_tx,
      server_info: server_info.clone(),
      exit: false,
      state: RunState::Overview,
    }
  }
}

impl<'a> Widget for &RunApp<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let title = Line::from(self.server_info.to_string().bold());
    let instructions = match self.state {
      RunState::Exiting => Line::from(vec![
        " Cancel".into(),
        "<q> ".into(),
        " Stop & Quit ".into(),
        "<y> ".into(),
      ]),
      RunState::Console => Line::from(vec![" Exit Console ".into(), "<Esc> ".blue().bold()]),
      RunState::Overview => Line::from(vec![
        " Console ".into(),
        "<ENTER> ".blue().bold(),
        " Stop & Quit ".into(),
        "<Q> ".blue().bold(),
      ]),
    };

    let mut block = Block::bordered()
      .title(title.centered())
      .title_bottom(instructions.centered())
      .border_set(border::THICK);

    match self.state {
      RunState::Exiting => block = block.red(),
      RunState::Console => block = block.yellow(),
      RunState::Overview => block = block.green(),
    };

    let logs = Text::from(self.logs.clone());

    let visible_height = area.height.saturating_sub(2) as usize;

    let scroll = if self.auto_scroll {
      self.logs.len().saturating_sub(visible_height) as u16
    } else {
      self.scroll
    };

    Paragraph::new(logs)
      .left_aligned()
      .block(block)
      .scroll((scroll, 0))
      .wrap(Wrap { trim: false })
      .render(area, buf);
  }
}
