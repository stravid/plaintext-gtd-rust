pub struct Task {
    pub text: String,
}

impl Task {
    pub fn default(text: String) -> Self {
        Self { text }
    }
}
