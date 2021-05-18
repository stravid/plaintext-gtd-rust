mod store;
mod task;
mod terminal;
mod tui;
mod uuid;

fn main() {
    let path_to_database = std::path::Path::new("plaintext-gtd.db");
    let sqlite_flags = sqlite::OpenFlags::new().set_create().set_read_write();
    let connection = match sqlite::Connection::open_with_flags(path_to_database, sqlite_flags) {
        Ok(connection) => connection,
        Err(error) => panic!(
            "Problem opening SQLite database `{}`: {}",
            path_to_database.display(),
            error
        ),
    };

    let terminal = terminal::Terminal::default();
    let mut store = store::Store::default(connection);
    let mut tui = tui::Tui::default(&terminal, &mut store);

    tui.run();
}
