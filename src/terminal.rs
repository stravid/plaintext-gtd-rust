use termion::event::Key;
use std::io;
use termion::input::TermRead;

pub struct Terminal {}

impl Terminal {
    pub fn default() -> Self {
        Self {}
    }

    pub fn get_next_key(&self) -> Key {
        match io::stdin().lock().keys().next() {
            None => panic!("Terminal Error: No key pressed"),
            Some(key) => match key {
                Ok(key) => key,
                Err(error) => panic!("Terminal Error: {}", error),
            },
        }
    }
}