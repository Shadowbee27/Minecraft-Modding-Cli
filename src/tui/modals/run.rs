use color_eyre::{Result, eyre::Context};
use crossterm::{
  event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
  terminal::enable_raw_mode,
};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::{Constraint, Direction, Layout, Rect},
  style::Stylize,
  symbols::border,
  text::{Line, Text},
  widgets::{Block, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use std::{
  fmt,
  io::{BufReader, Write},
  path::PathBuf,
  process::{Child, ChildStdin, Command, Stdio},
  sync::{
    Arc, RwLock,
    mpsc::{self, Sender},
  },
  thread::{self},
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

pub fn init_run_server_modal(exe_args: ExeArgs, server_info: ServerDetails) -> Result<()> {
  enable_raw_mode()?;
  ratatui::run(|terminal| RunApp::new(exe_args, server_info).run(terminal))?;
  Ok(())
}

fn update_logs(logs: Arc<RwLock<Vec<String>>>, rx: mpsc::Receiver<String>) {
  while let Ok(line) = rx.recv() {
    logs.write().unwrap().push(line);
  }
}

#[derive(Clone, Debug)]
pub struct ConsoleInput {
  state: RunState,
  input: String,
  character_index: usize,
  messages: Vec<String>,
}

#[derive(Debug)]
pub struct RunApp {
  rows: Vec<(String, LogType)>,
  rows_done: usize,
  rows_width: usize,
  input: ConsoleInput,
  auto_scroll: bool,
  scroll: u16,
  logs: Arc<RwLock<Vec<String>>>,
  exe_args: ExeArgs,
  server: Option<Child>,
  server_stdin: Option<ChildStdin>,
  server_info: ServerDetails,
  exit: bool,
  state: RunState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum LogType {
  INFO,
  WARN,
  ERROR,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunState {
  Exiting,  // Popup asking the user if the server should be stopped
  Console,  // the user is able to input commands into to the server console
  Overview, // Just showing the logs with the option to exit or enter console mode
  Crashed,  // The server stopped unexpectedly
}

impl RunApp {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    let (tx, rx) = mpsc::channel::<String>();
    self.start_process(self.exe_args.clone(), tx);

    let logs_writer = Arc::clone(&self.logs);
    thread::spawn(move || {
      update_logs(logs_writer, rx);
    });

    loop {
      if !self.input.messages.is_empty() {
        // Handle command
        for mut cmd in std::mem::take(&mut self.input.messages) {
          if !cmd.starts_with("/") {
            cmd = format!("/say {}", cmd);
          }
          if let Some(stdin) = self.server_stdin.as_mut()
            && writeln!(stdin, "{}", cmd)
              .and_then(|_| stdin.flush())
              .is_err()
          {
            self.state = RunState::Crashed;
          }
        }
      }

      terminal.draw(|frame| self.draw(frame))?;

      if event::poll(Duration::from_millis(50))? {
        self.handle_events()?;
      }

      if self.state != RunState::Crashed
        && !self.exit
        && let Some(server) = &mut self.server
        && let Ok(Some(_status)) = server.try_wait()
      {
        self.state = RunState::Crashed;
        self.server_stdin = None;
      }

      if self.exit {
        if let Some(stdin) = self.server_stdin.as_mut() {
          let _ = writeln!(stdin, "stop");
          let _ = stdin.flush();
        } else if let Some(child) = self.server.as_mut() {
          child.kill()?;
        }

        if let Some(proc) = self.server.as_mut() {
          proc.wait()?;
        }

        break;
      }
    }

    Ok(())
  }

  fn draw(&mut self, frame: &mut Frame) {
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Min(5), Constraint::Length(3)])
      .split(frame.area());

    self.sync_rows(chunks[0].width.saturating_sub(2) as usize);
    let max = self.max_scroll(chunks[0]);

    if self.auto_scroll {
      self.scroll = max;
    } else {
      self.scroll = self.scroll.min(max);
      if self.scroll == max {
        self.auto_scroll = true;
      }
    }

    self.input.state = self.state;
    frame.render_widget(&*self, chunks[0]);
    frame.render_widget(self.input.clone(), chunks[1]);
  }

  fn handle_events(&mut self) -> Result<()> {
    match event::read()? {
      // it's important to check that the event is a key press event as
      // crossterm also emits key release and repeat events on Windows.
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)?
      }
      _ => {}
    };
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
    match self.state {
      RunState::Exiting => match key_event.code {
        KeyCode::Char('q') | KeyCode::Esc => self.state = RunState::Overview,
        KeyCode::Char('y') => self.exit = true,
        _ => {}
      },

      RunState::Console if key_event.kind == KeyEventKind::Press => match key_event.code {
        KeyCode::Enter => {
          self.submit_message();
          self.state = RunState::Overview;
        }
        KeyCode::Char(to_insert) => self.enter_char(to_insert),
        KeyCode::Backspace => self.delete_char(),
        KeyCode::Left => self.move_cursor_left(),
        KeyCode::Right => self.move_cursor_right(),
        KeyCode::Esc => self.state = RunState::Overview,
        _ => {}
      },

      RunState::Overview => match key_event.code {
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
      },
      RunState::Console => {}
      RunState::Crashed => match key_event.code {
        KeyCode::Char('q') => self.exit = true,
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
      },
    }

    Ok(())
  }

  fn start_process(&mut self, exe_args: ExeArgs, sender: Sender<String>) {
    #[allow(clippy::zombie_processes)]
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
    self.server_stdin = Some(child.stdin.take().unwrap());

    Self::read_logs(stdout, sender.clone());
    Self::read_logs(stderr, sender.clone());
  }

  fn read_logs<R: Read + Send + 'static>(stream: R, tx: Sender<String>) {
    thread::spawn(move || {
      for line in BufReader::new(stream).lines() {
        match line {
          Ok(line) => {
            let _ = tx.send(format!("{line}"));
          }
          Err(_) => break,
        }
      }
    });
  }

  fn move_cursor_left(&mut self) {
    let cursor_moved_left = self.input.character_index.saturating_sub(1);
    self.input.character_index = self.clamp_cursor(cursor_moved_left);
  }

  fn move_cursor_right(&mut self) {
    let cursor_moved_right = self.input.character_index.saturating_add(1);
    self.input.character_index = self.clamp_cursor(cursor_moved_right);
  }

  fn enter_char(&mut self, new_char: char) {
    let index = self.byte_index();
    self.input.input.insert(index, new_char);
    self.move_cursor_right();
  }

  /// Returns the byte index based on the character position.
  ///
  /// Since each character in a string can contain multiple bytes, it's necessary to calculate
  /// the byte index based on the index of the character.
  fn byte_index(&self) -> usize {
    self
      .input
      .input
      .char_indices()
      .map(|(i, _)| i)
      .nth(self.input.character_index)
      .unwrap_or(self.input.input.len())
  }

  fn delete_char(&mut self) {
    let is_not_cursor_leftmost = self.input.character_index != 0;
    if is_not_cursor_leftmost {
      let current_index = self.input.character_index;
      let from_left_to_current_index = current_index - 1;

      // Getting all characters before the selected character.
      let before_char_to_delete = self.input.input.chars().take(from_left_to_current_index);
      // Getting all characters after selected character.
      let after_char_to_delete = self.input.input.chars().skip(current_index);

      // Put all characters together except the selected one.
      // By leaving the selected one out, it is forgotten and therefore deleted.
      self.input.input = before_char_to_delete.chain(after_char_to_delete).collect();
      self.move_cursor_left();
    }
  }

  fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
    new_cursor_pos.clamp(0, self.input.input.chars().count())
  }

  const fn reset_cursor(&mut self) {
    self.input.character_index = 0;
  }

  fn submit_message(&mut self) {
    self.input.messages.push(self.input.input.clone());
    self.input.input.clear();
    self.reset_cursor();
  }

  fn sync_rows(&mut self, width: usize) {
    let logs = self.logs.read().unwrap();
    if width != self.rows_width {
      self.rows.clear();
      self.rows_done = 0;
      self.rows_width = width;
    }
    self.rows.extend(
      Self::wrap_rows(&logs[self.rows_done..], width)
        .into_iter()
        .map(|row| {
          if row.contains("ERROR") {
            (row, LogType::ERROR)
          } else if row.contains("WARN") {
            (row, LogType::WARN)
          } else {
            (row, LogType::INFO)
          }
        }),
    );
    self.rows_done = logs.len();
  }

  fn max_scroll(&self, area: Rect) -> u16 {
    let visible = area.height.saturating_sub(2) as usize;
    self
      .rows
      .len()
      .saturating_sub(visible)
      .min(u16::MAX as usize) as u16
  }

  fn wrap_rows(logs: &[String], width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut rows = Vec::new();
    for line in logs {
      let clean: Vec<char> = line
        .chars()
        .map(|c| if c == '\t' { ' ' } else { c })
        .filter(|c| !c.is_control())
        .collect();
      if clean.is_empty() {
        rows.push(String::new());
      } else {
        for chunk in clean.chunks(width) {
          rows.push(chunk.iter().collect());
        }
      }
    }
    rows
  }

  fn new(exe_args: ExeArgs, server_info: ServerDetails) -> Self {
    RunApp {
      input: ConsoleInput {
        state: RunState::Overview,
        input: String::new(),
        messages: Vec::new(),
        character_index: 0,
      },
      auto_scroll: true,
      scroll: 0,
      server: None,
      server_stdin: None,
      exe_args,
      logs: Arc::new(RwLock::new(vec![])),
      server_info: server_info.clone(),
      exit: false,
      state: RunState::Overview,
      rows: Vec::new(),
      rows_done: 0,
      rows_width: 0,
    }
  }
}

impl Widget for &RunApp {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let title = Line::from(self.server_info.to_string().bold());

    let mut block = Block::bordered()
      .title(title.centered())
      .border_set(border::THICK);

    match self.state {
      RunState::Exiting | RunState::Crashed => block = block.red(),
      _ => block = block.green(),
    };

    let visible = area.height.saturating_sub(2) as usize;
    let start = (self.scroll as usize).min(self.rows.len());
    let end = (start + visible).min(self.rows.len());

    let logs = Text::from(
      self.rows[start..end]
        .iter()
        .map(|r| {
          let mut line = Line::from(r.0.as_str());

          match r.1 {
            LogType::INFO => line = line.white(),
            LogType::WARN => line = line.yellow(),
            LogType::ERROR => line = line.red(),
          }

          line
        })
        .collect::<Vec<Line>>(),
    );

    Paragraph::new(logs)
      .left_aligned()
      .block(block)
      .render(area, buf);
  }
}

impl Widget for ConsoleInput {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let mut block = Block::bordered()
      .border_set(border::PLAIN)
      .title(Line::from(" Console ").centered());

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
      RunState::Crashed => Line::from(vec![" Stop & Quit ".into(), "<Q> ".blue().bold()]),
    };
    block = block.title_bottom(instructions.centered());

    match self.state {
      RunState::Exiting => block = block.red(),
      RunState::Overview => block = block.green(),
      RunState::Console => block = block.yellow(),
      RunState::Crashed => block = block.red(),
    };

    Paragraph::new(self.input).block(block).render(area, buf);
  }
}
