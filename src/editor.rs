use crate::{Terminal, Document};
// use crate::terminal::Terminal; // 如果 main.rs 中没有使用 pub use terminal::Terminal; 我们必须这样导入
use termion::{event::Key, cursor::Up};
use crate::Row;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
  pub x: usize, // 这里使用uszie是为了让终端尽量多的文档行数
  pub y: usize,
}

pub struct Editor {
  should_quit: bool,
  terminal: Terminal,
  document: Document,
  cursor_position: Position, // 光标在当前文档中的位置，而不是屏幕上的位置
  offset: Position,
}

impl Editor {
  pub fn default() -> Self {
    let args: Vec<String> = env::args().collect();
    let document = if args.len() > 1 {
      let filename = &args[1];
      Document::open(&filename).unwrap_or_default()
    } else {
      Document::default()
    };
    Editor {
      terminal: Terminal::default().expect("Failed to initialize terminal"),
      document,
      cursor_position: Position::default(),
      offset: Position::default(),
      should_quit: false
    }
  }

  /// 运行入口
  pub fn run(&mut self) {
    loop {
      if let Err(error) = self.refresh_screen() {
        die(error);
      }
      if self.should_quit {
        break;
      }
      if let Err(error) = self.process_keypress() {
        die(error);
      }
    }
  }

  /// 处理按键操作
  fn process_keypress(&mut self) -> Result<(), std::io::Error> {
    let process_key = Terminal::read_key()?;

    match process_key {
      Key::Ctrl('c') => self.should_quit = true,
      Key::Up
      | Key::Down
      | Key::Left
      | Key::Right
      | Key::PageUp
      | Key::PageDown
      | Key::End
      | Key::Home => self.move_cursor(process_key),
      _ => ()
    }
    self.scroll();
    Ok(())
  }

  fn scroll(&mut self) {
    let Position { x, y } = self.cursor_position;
    let width = self.terminal.size().width as usize;
    let height = self.terminal.size().height as usize;
    let mut offset = &mut self.offset;
    if y < offset.y {
      offset.y = y;
    } else if y >= offset.y.saturating_add(height) {
      offset.y = y.saturating_sub(height).saturating_add(1);
    }
    if x < offset.x {
      offset.x = x;
    } else if x >= offset.x.saturating_add(width) {
      offset.x = x.saturating_sub(width).saturating_add(1);
    }
  }

  /// 移动光标
  fn move_cursor(&mut self, key: Key) {
    let Position { mut x, mut y } = self.cursor_position;
    let size = self.terminal.size();
    let height = self.document.len();
    let width = if let Some(row) = self.document.row(y) {
      row.len()
    } else {
      0
    };
    
    match key {
      Key::Up => y = y.saturating_sub(1),
      Key::Down => {
        if y < height {
          y = y.saturating_add(1);
        }
      },
      Key::Left => x = x.saturating_sub(1),
      Key::Right => {
        if x < width {
          x = x.saturating_add(1);
        }
      },
      Key::PageUp => y = 0,
      Key::PageDown => y = height,
      Key::Home => x = 0,
      Key::End => x = width,
      _ => (),
    }
    self.cursor_position = Position { x, y };
  }

  /// 刷新终端并打印退出信息
  fn refresh_screen(&self) -> Result<(), std::io::Error> {
    Terminal::cursor_hide();
    Terminal::cursor_position(&Position::default());
    if self.should_quit {
      // 退出前清空所有屏幕
      Terminal::clear_screen();
      println!("Goodbye.\r");
    } else {
      self.draw_rows();
      Terminal::cursor_position(&Position {
        x: self.cursor_position.x.saturating_sub(self.offset.x),
        y: self.cursor_position.y.saturating_sub(self.offset.y),
      });
    }
    Terminal::cursor_show();
    Terminal::flush()
  }

  fn draw_welcome_message(&self) {
    let mut welcome_message = format!("RS-EDIT -- version {}", VERSION);
    let width = self.terminal.size().width as usize;
    let len = welcome_message.len();
    let padding = width.saturating_sub(len) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    welcome_message = format!("~{}{}", spaces, welcome_message);
    welcome_message.truncate(width);
    println!("{}\r", welcome_message);
  }

  fn draw_row(&self, row: &Row) {
    let width = self.terminal.size().width as usize;
    let start = self.offset.x;
    let end = self.offset.x + width;
    let row = row.render(start, end);
    println!("{}\r", row);
  }

  /// 针对终端高度在每行首输出~符号
  fn draw_rows(&self) {
    let height = self.terminal.size().height;

    for terminal_row in 0..height - 1 {
      Terminal::clear_current_line(); // 绘制当前行时先清空当前行
      if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
        self.draw_row(row);
      } else if self.document.is_empty() && terminal_row == height / 3 {
        self.draw_welcome_message();
      } else {
        println!("~\r");
      }
    }
  }
}


fn die(e: std::io::Error) {
  Terminal::clear_screen();
  panic!("{}", e);
}

