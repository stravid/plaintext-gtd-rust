use termion::event::Key;

use crate::task::{State, Task};
use crate::terminal;
use crate::terminal::Print;

pub struct Tui<'a> {
    mode: u8,
    terminal: &'a terminal::Terminal,
    index: u32,
}

impl<'a> Tui<'a> {
    pub fn default(terminal: &'a terminal::Terminal) -> Self {
        Self {
            mode: 1,
            terminal,
            index: 0,
        }
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
                    Key::Down => {
                        self.index = if self.index == (tasks.len() - 1) as u32 {
                            self.index
                        } else {
                            self.index + 1
                        }
                    }
                    Key::Char('d') => self.change_task(&mut tasks, State::Done),
                    Key::Char('i') => self.change_task(&mut tasks, State::InProgress),
                    Key::Char('t') => self.change_task(&mut tasks, State::Todo),
                    Key::Char('x') => self.change_task(&mut tasks, State::Discarded),
                    _ => (),
                }
            } else if self.mode == 2 {
                match key {
                    Key::Esc => self.mode = 1,
                    Key::Backspace => {
                        input.pop();
                    }
                    Key::Char(char) => {
                        if char == '\n' {
                            self.mode = 1;

                            if !input.trim().is_empty() {
                                tasks.push(Task::default(input.clone()));
                            }

                            input = String::from("");
                        } else {
                            input.push(char);
                        }
                    }
                    _ => (),
                }
            }
        }

        self.terminal.show_cursor();
        self.terminal.clear();
    }

    fn print_list(&mut self, tasks: &[Task]) {
        let mut lines = vec![
            vec![Print::Text("List of tasks")],
            vec![Print::Text("-------------")],
            vec![],
        ];

        if tasks.is_empty() {
            lines.push(vec![Print::Text("No tasks")])
        }

        for (i, action) in tasks.iter().enumerate() {
            let (color, sign) = match action.state {
                State::Todo => (Print::Blue, Print::Text("- ")),
                State::Done => (Print::Green, Print::Text("+ ")),
                State::InProgress => (Print::Yellow, Print::Text("~ ")),
                State::Discarded => (Print::Red, Print::Text("+ ")),
            };

            if i == self.index as usize {
                lines.push(vec![
                    color,
                    sign,
                    Print::WhiteBackground,
                    Print::Black,
                    Print::Text(&action.text),
                    Print::ResetBackground,
                    Print::ResetForeground,
                ]);
            } else {
                lines.push(vec![color, sign, Print::White, Print::Text(&action.text)]);
            }
        }

        lines.push(vec![]);
        lines.push(vec![Print::Text("Press ESC to quit.")]);
        lines.push(vec![Print::Text("Press ENTER to enter new task.")]);
        lines.push(vec![Print::Text("Press D to mark task as done.")]);
        lines.push(vec![Print::Text("Press I to mark task as in-progress.")]);
        lines.push(vec![Print::Text("Press T to mark task as to-do.")]);
        lines.push(vec![Print::Text("Press X to mark task as discarded.")]);

        self.terminal.hide_cursor();
        self.terminal.print(lines);
    }

    fn print_input(&mut self, input: &str) {
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

    fn change_task(&self, tasks: &mut Vec<Task>, state: State) {
        let task = tasks.remove(self.index as usize);
        tasks.insert(
            self.index as usize,
            Task {
                text: task.text,
                state,
            },
        );
    }
}
