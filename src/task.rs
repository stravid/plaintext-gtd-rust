use crate::uuid::Uuid;

#[derive(Clone)]
pub enum State {
    Todo,
    Done,
    InProgress,
    Discarded,
}

#[derive(Clone)]
pub struct Task {
    pub uuid: Uuid,
    pub text: String,
    pub state: State,
}

impl Task {
    pub fn default(text: String) -> Self {
        Self {
            uuid: Uuid::new(),
            text,
            state: State::Todo,
        }
    }
}

impl sqlite::Readable for State {
    fn read(statement: &sqlite::Statement, i: usize) -> sqlite::Result<State> {
        let result = statement.read::<String>(i).unwrap();

        if result == "todo" {
            sqlite::Result::Ok(State::Todo)
        } else if result == "done" {
            sqlite::Result::Ok(State::Done)
        } else if result == "inprogress" {
            sqlite::Result::Ok(State::InProgress)
        } else if result == "discarded" {
            sqlite::Result::Ok(State::Discarded)
        } else {
            sqlite::Result::Err(sqlite::Error {
                code: Option::None,
                message: Option::Some(format!("Unknown task state {}", result)),
            })
        }
    }
}

impl sqlite::Bindable for State {
    fn bind(self, statement: &mut sqlite::Statement, i: usize) -> sqlite::Result<()> {
        let string = match self {
            State::Todo => String::from("todo"),
            State::Done => String::from("done"),
            State::InProgress => String::from("inprogress"),
            State::Discarded => String::from("discarded"),
        };

        statement.bind(i, &string[..]).unwrap();
        Ok(())
    }
}