pub enum State {
    Todo,
    Done,
    InProgress,
    Discarded,
}

pub struct Task {
    pub text: String,
    pub state: State,
}

impl Task {
    pub fn default(text: String) -> Self {
        Self { text, state: State::Todo }
    }
}
