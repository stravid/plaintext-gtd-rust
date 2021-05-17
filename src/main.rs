mod tui;
use tui::Tui;
mod terminal;
use terminal::Terminal;
mod store;
mod task;
mod uuid;

use store::Store;
use std::path::Path;
use sqlite::OpenFlags;

fn main() {
    let connection = sqlite::Connection::open_with_flags(Path::new("plaintext-gtd.db"), OpenFlags::new().set_create().set_read_write()).unwrap();
    let mut store = Store::default(connection);
    let terminal = Terminal::default();
    let mut tui = Tui::default(&terminal, &mut store);
    tui.run();
}
