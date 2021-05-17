use crate::task::Task;
use sqlite::State;
use crate::task;
use crate::uuid::Uuid;

pub struct Store {
    connection: sqlite::Connection,
}

impl Store {
    pub fn default(connection: sqlite::Connection) -> Self {
        Self::migrate(&connection);

        Self {
            connection,
        }
    }

    pub fn query_tasks(&self) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];
        let mut statement = self.connection.prepare(r"
            SELECT
                uuid,
                text,
                state
            FROM
                tasks
            ORDER BY
                rowid ASC
            ;
        ").unwrap();

        while let State::Row =statement.next().unwrap() {
            let uuid = statement.read::<Uuid>(0).unwrap();
            let text = statement.read::<String>(1).unwrap();
            let state = statement.read::<task::State>(2).unwrap();

            tasks.push(Task { uuid, text, state });
        }

        tasks
    }

    pub fn persist_task(&mut self, task: Task) {
        let mut statement = self.connection.prepare(r"
            INSERT INTO
                tasks (
                    uuid,
                    text,
                    state
                )
            VALUES (
                ?,
                ?,
                ?
            )
            ON CONFLICT(uuid) DO UPDATE SET text = ?, state = ?;
            ;
        ").unwrap();

        statement.bind(1, task.uuid).unwrap();
        statement.bind(2, &task.text[..]).unwrap();
        statement.bind(3, task.state.clone()).unwrap();
        statement.bind(4, &task.text[..]).unwrap();
        statement.bind(5, task.state).unwrap();

        statement.next().unwrap();
    }

    fn migrate(connection: &sqlite::Connection) {
        let mut version_statement = connection.prepare("PRAGMA user_version;").unwrap();
        version_statement.next().unwrap();
        let version = version_statement.read::<i64>(0).unwrap();

        if version == 0 {
            connection.execute(r"
                CREATE TABLE tasks (
                    uuid TEXT NOT NULL,
                    text TEXT NOT NULL CHECK (LENGTH(text) > 0),
                    state TEXT NOT NULL CHECK (state IN ('todo', 'inprogress', 'done', 'discarded')),
                    PRIMARY KEY (uuid)
                );

                PRAGMA user_version = 1;
            ").unwrap();
        }
    }
}
