use termion::event::Key;

use crate::task::Task;
use crate::terminal;
use crate::terminal::Print;

pub struct Tui<'a> {
    mode: u8,
    terminal: &'a terminal::Terminal,
    index: u32,
}

impl<'a > Tui<'a> {
    pub fn default(terminal: &'a terminal::Terminal) -> Self {
       Self { mode: 1, terminal, index: 0 }
    }

    pub fn run(&mut self) {
        let mut input = String::from("");
        let mut tasks = vec![Task::default(String::from("Learn Rust"))];

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
                    Key::Up => self.index = if self.index == 0 { 0 } else { self.index - 1 },
                    Key::Down => self.index = if self.index == (tasks.len() - 1) as u32 { self.index } else { self.index + 1 },
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
                                tasks.push(Task::default(input.clone()));
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

    fn print_list(&mut self, tasks: &Vec<Task>) {
        let mut lines = vec![
            vec![Print::Text("List of tasks")],
            vec![Print::Text("-------------")],
            vec![],
        ];

        if tasks.len() == 0 {
            lines.push(vec![Print::Text("No tasks")])
        }

        for (i, action) in tasks.iter().enumerate() {
            if i == self.index as usize {
                lines.push(vec![Print::WhiteBackground, Print::Blue, Print::Text("- "), Print::Black, Print::Text(&action.text), Print::ResetBackground, Print::ResetForeground]);
            } else {
                lines.push(vec![Print::Blue, Print::Text("- "), Print::White, Print::Text(&action.text)]);
            }
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
        self.terminal.move_cursor((input.len() + 1) as u16, 2);
    }
}