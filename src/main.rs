use std::io::{self, stdout};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

fn main() {
    let actions = vec!["Learn Rust", "Learn Go", "Convince Thomas"];
    let number_of_actions = actions.len();

    match number_of_actions {
        0 => println!("Nothing left to do, great job!"),
        1 => println!("There is one action:"),
        _ => println!("There are {} actions:", number_of_actions),
    }

    for action in actions.iter() {
        println!("{}", action);
    }

    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Esc => break,
                key => println!("{:?}", key),
            },
            Err(error) => panic!(error),
        }
    }
}
