use std::io::{self, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}       

impl Editor {
  pub fn default() -> Self {
    Editor {  }
  }
  pub fn run(&self) {
    let _stdout = stdout().into_raw_mode().unwrap();

    loop {
      if let Err(error) = self.process_keypress() {

      }
    }
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                    } else {
                        println!("{:?} ({})", c as u8, c);
                    }
                },
                Key::Ctrl('c') => break,
                _ => println!("{:?}\r", key)
            },
            Err(err) => die(err),
        }
    }
  }
  fn process_keypress(&self) -> Result<(), std::io::Error> {
    let process_key = read_key()?;
    match process_key {
      Key::Ctrl('c') => panic!("Program end"),
      _ => (),
    }
    Ok(())
  }
}

fn die(e: std::io::Error) {
  panic!("{}", e);
}

fn read_key() -> Result<Key, std::io::Error> {
  loop {
    if let Some(key) = io:stdin().lock()
  }
}
