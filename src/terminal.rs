use termion::event::Key;
use std::io;
use termion::input::TermRead;
use std::io::Write;

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

    pub fn clear(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.flush();
    }

    pub fn show_cursor(&self) {
        print!("{}", termion::cursor::Show);
        self.flush();
    }

    pub fn hide_cursor(&self) {
        print!("{}", termion::cursor::Hide);
        self.flush();
    }

    fn flush(&self) {
        match io::stdout().flush() {
            Ok(_) => {}
            Err(error) => println!("Terminal Error: {}", error),
        }
    }
}