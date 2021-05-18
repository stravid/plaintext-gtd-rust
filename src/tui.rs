use crate::store::Store;
use crate::task::{State, Task};
use crate::terminal::print::Instruction::{Background, Foreground, Text};
use crate::{store, terminal};
use terminal::print::Color;
use termion::event::Key;

pub struct Tui<'a> {
    mode: Mode,
    terminal: &'a terminal::Terminal,
    index: u32,
    store: &'a mut store::Store,
}

enum Mode {
    List,
    New,
    Edit,
}

impl<'a> Tui<'a> {
    pub fn default(terminal: &'a terminal::Terminal, store: &'a mut Store) -> Self {
        Self {
            mode: Mode::List,
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

            match self.mode {
                Mode::List => self.print_list(&tasks),
                Mode::New => self.print_input(&input),
                Mode::Edit => self.print_edit(&input),
            }

            let key = self.terminal.get_next_key();

            match self.mode {
                Mode::List => match key {
                    Key::Esc => break,
                    Key::Char('\n') => self.mode = Mode::New,
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
                            self.mode = Mode::Edit;
                        }
                    }
                    _ => (),
                },
                Mode::New => match key {
                    Key::Esc => self.mode = Mode::List,
                    Key::Backspace => {
                        input.pop();
                    }
                    Key::Char(char) => {
                        if char == '\n' {
                            self.mode = Mode::List;

                            if !input.trim().is_empty() {
                                self.store.persist_task(Task::default(input.clone()));
                            }

                            input = String::from("");
                        } else {
                            input.push(char);
                        }
                    }
                    _ => (),
                },
                Mode::Edit => match key {
                    Key::Esc => self.mode = Mode::List,
                    Key::Backspace => {
                        input.pop();
                    }
                    Key::Char(char) => {
                        if char == '\n' {
                            self.mode = Mode::List;

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
                },
            }
        }

        self.terminal.show_cursor();
        self.terminal.clear();
    }

    fn print_list(&mut self, tasks: &[Task]) {
        let mut lines = vec![];

        if tasks.is_empty() {
            lines.push(vec![Text("No tasks")])
        }

        for (i, action) in tasks.iter().enumerate() {
            let (color, sign) = match action.state {
                State::Todo => (Foreground(Color::Blue), Text("- ")),
                State::Done => (Foreground(Color::Green), Text("+ ")),
                State::InProgress => (Foreground(Color::Yellow), Text("~ ")),
                State::Discarded => (Foreground(Color::Red), Text("+ ")),
            };

            if i == self.index as usize {
                lines.push(vec![
                    color,
                    sign,
                    Background(Color::White),
                    Foreground(Color::Black),
                    Text(&action.text),
                    Background(Color::Reset),
                    Foreground(Color::Reset),
                ]);
            } else {
                lines.push(vec![
                    color,
                    sign,
                    Foreground(Color::White),
                    Text(&action.text),
                ]);
            }
        }

        lines.push(vec![]);
        lines.push(vec![Text("Press ESC to quit.")]);
        lines.push(vec![Text("Press ENTER to enter new task.")]);
        lines.push(vec![Text("Use UP/DOWN to move task selection.")]);
        lines.push(vec![Text("Press D to mark task as done.")]);
        lines.push(vec![Text("Press I to mark task as in-progress.")]);
        lines.push(vec![Text("Press T to mark task as to-do.")]);
        lines.push(vec![Text("Press X to mark task as discarded.")]);
        lines.push(vec![Text("Press E to edit task.")]);

        self.terminal.hide_cursor();
        self.terminal.print(lines);
    }

    fn print_input(&mut self, input: &str) {
        let lines = vec![
            vec![Text("New task:")],
            vec![Text(input)],
            vec![],
            vec![Text("Press ESC to switch to task list.")],
            vec![Text("Press ENTER to add task.")],
        ];

        self.terminal.show_cursor();
        self.terminal.print(lines);
        self.terminal.move_cursor((input.len() + 1) as u16, 2);
    }

    fn print_edit(&mut self, input: &str) {
        let lines = vec![
            vec![Text("Edit task:")],
            vec![Text(input)],
            vec![],
            vec![Text("Press ESC to switch to task list.")],
            vec![Text("Press ENTER to update task.")],
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
