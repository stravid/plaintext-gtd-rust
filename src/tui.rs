use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use crate::terminal;
use crate::terminal::Print;

pub struct Tui<'a> {
    mode: u8,
    terminal: &'a terminal::Terminal,
}

impl<'a > Tui<'a> {
    pub fn default(terminal: &'a terminal::Terminal) -> Self {
       Self { mode: 1, terminal }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        let mut input = String::from("");
        let mut tasks = vec![String::from("Learn Rust")];

        loop {
            self.terminal.clear();

            if self.mode == 1 {
                self.print_list(&tasks);
            }

            if self.mode == 2 {
                self.print_input(&input);
            }

            let key = self.terminal.get_next_key();

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

        self.terminal.show_cursor();
        self.terminal.clear();
    }

    fn print_list(&mut self, tasks: &Vec<String>) {
        let mut lines = vec![
            vec![Print::Text("List of tasks")],
            vec![Print::Text("-------------")],
            vec![],
        ];

        if tasks.len() == 0 {
            lines.push(vec![Print::Text("No tasks")])
        }

        for action in tasks.iter() {
            lines.push(vec![Print::Text(action)]);
        }

        lines.push(vec![]);
        lines.push(vec![Print::Text("Press ESC to quit.")]);
        lines.push(vec![Print::Text("Press ENTER to enter new task.")]);

        self.terminal.hide_cursor();
        self.terminal.print(lines);
    }

    fn print_input(&mut self, input: &String) {
        let lines = vec![
            vec![Print::Text("New task:")],
            vec![Print::Text(input)],
            vec![],
            vec![Print::Text("Press ESC to switch to task list.")],
            vec![Print::Text("Press ENTER to add task.")],
        ];

        self.terminal.show_cursor();
        self.terminal.print(lines);

        print!("{}", termion::cursor::Goto((input.len() + 1) as u16, 2));
        io::stdout().flush();
    }
}