use uuid::Uuid;

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
            uuid: Uuid::new_v4(),
            text,
            state: State::Todo,
        }
    }
}
