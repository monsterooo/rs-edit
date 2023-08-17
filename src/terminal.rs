use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::Position;


/// 终端大小结构
pub struct Size {
  pub width: u16,
  pub height: u16,
}

/// 终端结构
pub struct Terminal {
  size: Size, // 不会公开此属性，因为不允许外部调用改变大小
  _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
  pub fn default() -> Result<Self, std::io::Error> {
    let size = termion::terminal_size()?;
    Ok(Self {
      size: Size {
        width: size.0,
        height: size.1,
      },
      _stdout: stdout().into_raw_mode()?,
    })
  }

  /// 但是提供size属性，可以让外部访问到终端大小
  pub fn size(&self) -> &Size {
    &self.size
  }

  /// 清除屏幕
  pub fn clear_screen() {
    print!("{}", termion::clear::All);
  }

  /// 移动光标位置
  pub fn cursor_position(position: &Position) {
    let Position { mut x, mut y } = position;

    x = x.saturating_add(1);
    y = y.saturating_add(1);

    let x = x as u16;
    let y = y as u16;
    
    print!("{}", termion::cursor::Goto(x, y));
  }

  /// 刷新终端
  pub fn flush() -> Result<(), std::io::Error> {
    io::stdout().flush()
  }

  /// 读取终端的键
  pub fn read_key() -> Result<Key, std::io::Error> {
    loop {
      if let Some(key) = io::stdin().lock().keys().next() {
        return key;
      }
    }
  }

  /// 隐藏光标
  pub fn cursor_hide() {
    print!("{}", termion::cursor::Hide);
  }

  /// 显示光标
  pub fn cursor_show() {
    print!("{}", termion::cursor::Show);
  }

  pub fn clear_current_line() {
    print!("{}", termion::clear::CurrentLine);
  }
  
}