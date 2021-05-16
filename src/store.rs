use crate::task::Task;

pub struct Store {
    tasks: Vec<Task>,
}

impl Store {
    pub fn default() -> Self {
        Self {
            tasks: vec![Task::default(String::from("Learn Rust"))],
        }
    }

    pub fn query_tasks(&self) -> Vec<Task> {
        self.tasks.to_vec()
    }

    pub fn persist_task(&mut self, task: Task) {
        let index = self.tasks.iter().position(|t| t.uuid == task.uuid);

        match index {
            None => self.tasks.push(task),
            Some(index) => {
                self.tasks.remove(index);
                self.tasks.insert(index, task);
            }
        }
    }
}
