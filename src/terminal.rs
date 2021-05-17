use std::io;
use std::io::{stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::terminal::print::{Instruction, Color};

pub struct Terminal {
    _stdout: RawTerminal<Stdout>,
}

pub mod print {
    pub enum Instruction<'a> {
        Text(&'a str),
        Foreground(Color),
        Background(Color),
    }

    pub enum Color {
        Blue,
        White,
        Black,
        Green,
        Yellow,
        Red,
        Reset,
    }
}

impl Terminal {
    pub fn default() -> Self {
        Self {
            _stdout: stdout().into_raw_mode().unwrap(),
        }
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

    pub fn print(&self, lines: Vec<Vec<print::Instruction>>) {
        for line in lines {
            for instruction in line {
                match instruction {
                    Instruction::Text(text) => print!("{}", text),
                    Instruction::Foreground(Color::Blue) => print!("{}", termion::color::Fg(termion::color::Blue)),
                    Instruction::Foreground(Color::White) => print!("{}", termion::color::Fg(termion::color::White)),
                    Instruction::Foreground(Color::Black) => print!("{}", termion::color::Fg(termion::color::Black)),
                    Instruction::Foreground(Color::Green) => print!("{}", termion::color::Fg(termion::color::Green)),
                    Instruction::Foreground(Color::Yellow) => print!("{}", termion::color::Fg(termion::color::Yellow)),
                    Instruction::Foreground(Color::Red) => print!("{}", termion::color::Fg(termion::color::Red)),
                    Instruction::Foreground(Color::Reset) => print!("{}", termion::color::Fg(termion::color::Reset)),
                    Instruction::Background(Color::Blue) => print!("{}", termion::color::Bg(termion::color::Blue)),
                    Instruction::Background(Color::White) => print!("{}", termion::color::Bg(termion::color::White)),
                    Instruction::Background(Color::Black) => print!("{}", termion::color::Bg(termion::color::Black)),
                    Instruction::Background(Color::Green) => print!("{}", termion::color::Bg(termion::color::Green)),
                    Instruction::Background(Color::Yellow) => print!("{}", termion::color::Bg(termion::color::Yellow)),
                    Instruction::Background(Color::Red) => print!("{}", termion::color::Bg(termion::color::Red)),
                    Instruction::Background(Color::Reset) => print!("{}", termion::color::Bg(termion::color::Reset)),
                }
            }

            print!("\n\r")
        }
    }

    pub fn move_cursor(&self, column: u16, row: u16) {
        print!("{}", termion::cursor::Goto(column, row));
        self.flush();
    }

    fn flush(&self) {
        match io::stdout().flush() {
            Ok(_) => {}
            Err(error) => println!("Terminal Error: {}", error),
        }
    }
}
