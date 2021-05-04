use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

pub struct Tui {
    mode: u8,
}

impl Tui {
    pub fn default() -> Self {
       Self { mode: 1 }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        let mut input = String::from("");
        let mut tasks = vec![String::from("Learn Rust")];

        loop {
            print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
            io::stdout().flush();

            if self.mode == 1 {
                self.print_list(&tasks);
            }

            if self.mode == 2 {
                self.print_input(&input);
            }

            let key = io::stdin().lock().keys().next().unwrap().unwrap();

            if self.mode == 1 {
                match key {
                    Key::Esc => break,
                    Key::Char('\n') => self.mode = 2,
                    _ => (),
                }
            } else if self.mode == 2 {
                match key {
                    Key::Esc => self.mode = 1,
                    Key::Backspace => { input.pop(); ()},
                    Key::Char(char) => {
                        if char == '\n' {
                            self.mode = 1;

                            if input.trim().len() > 0 {
                                tasks.push(input.clone());
                            }

                            input = String::from("");
                        } else {
                            input.push(char);
                        }
                    },
                    _ => (),
                }
            }
        }

        print!("{}", termion::cursor::Show);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush();
    }

    fn print_list(&mut self, tasks: &Vec<String>) {
        print!("{}", termion::cursor::Hide);
        println!("List of tasks\r");
        println!("-------------\r");
        println!("\r");

        if tasks.len() == 0 {
            println!("No tasks\r");
        }

        for action in tasks.iter() {
            println!("{}\r", action);
        }

        println!("\r");
        println!("Press ESC to quit.\r");
        println!("Press ENTER to enter new task.\r");
    }

    fn print_input(&mut self, input: &String) {
        print!("{}", termion::cursor::Show);
        println!("New task:\r");
        println!("{}\r", input);
        println!("\r");
        println!("Press ESC to switch to task list.\r");
        println!("Press ENTER to add task.\r");
        print!("{}", termion::cursor::Goto((input.len() + 1) as u16, 2));
        io::stdout().flush();
    }
}