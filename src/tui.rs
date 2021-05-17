use termion::event::Key;

use crate::store::Store;
use crate::task::{State, Task};
use crate::terminal::Print;
use crate::{store, terminal};

pub struct Tui<'a> {
    mode: u8,
    terminal: &'a terminal::Terminal,
    index: u32,
    store: &'a mut store::Store,
}

impl<'a> Tui<'a> {
    pub fn default(terminal: &'a terminal::Terminal, store: &'a mut Store) -> Self {
        Self {
            mode: 1,
            terminal,
            index: 0,
            store,
        }
    }

    pub fn run(&mut self) {
        let mut input = String::from("");

        loop {
            let tasks = self.store.query_tasks();
            self.terminal.clear();

            if self.mode == 1 {
                self.print_list(&tasks);
            }

            if self.mode == 2 {
                self.print_input(&input);
            }

            if self.mode == 3 {
                self.print_edit(&input);
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
                    Key::Char('d') => self.change_task(&tasks, State::Done),
                    Key::Char('i') => self.change_task(&tasks, State::InProgress),
                    Key::Char('t') => self.change_task(&tasks, State::Todo),
                    Key::Char('x') => self.change_task(&tasks, State::Discarded),
                    Key::Char('e') => {
                        if !tasks.is_empty() {
                            let task = tasks.get(self.index as usize).unwrap();
                            input = task.text.clone();
                            self.mode = 3;
                        }
                    }
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
                                self.store.persist_task(Task::default(input.clone()));
                            }

                            input = String::from("");
                        } else {
                            input.push(char);
                        }
                    }
                    _ => (),
                }
            } else if self.mode == 3 {
                match key {
                    Key::Esc => self.mode = 1,
                    Key::Backspace => {
                        input.pop();
                    }
                    Key::Char(char) => {
                        if char == '\n' {
                            self.mode = 1;

                            if !input.trim().is_empty() {
                                let task = tasks.get(self.index as usize).unwrap();
                                self.store.persist_task(Task {
                                    text: input.clone(),
                                    ..task.clone()
                                });
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
        let mut lines = vec![];

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
        lines.push(vec![Print::Text("Use UP/DOWN to move task selection.")]);
        lines.push(vec![Print::Text("Press D to mark task as done.")]);
        lines.push(vec![Print::Text("Press I to mark task as in-progress.")]);
        lines.push(vec![Print::Text("Press T to mark task as to-do.")]);
        lines.push(vec![Print::Text("Press X to mark task as discarded.")]);
        lines.push(vec![Print::Text("Press E to edit task.")]);

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

    fn print_edit(&mut self, input: &str) {
        let lines = vec![
            vec![Print::Text("Edit task:")],
            vec![Print::Text(input)],
            vec![],
            vec![Print::Text("Press ESC to switch to task list.")],
            vec![Print::Text("Press ENTER to update task.")],
        ];

        self.terminal.show_cursor();
        self.terminal.print(lines);
        self.terminal.move_cursor((input.len() + 1) as u16, 2);
    }

    fn change_task(&mut self, tasks: &[Task], state: State) {
        let task = tasks.get(self.index as usize).unwrap();
        self.store.persist_task(Task {
            state,
            ..task.clone()
        });
    }
}
