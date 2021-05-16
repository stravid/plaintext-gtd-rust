mod tui;
use tui::Tui;
mod terminal;
use terminal::Terminal;
mod store;
mod task;
use store::Store;

fn main() {
    let mut store = Store::default();
    let terminal = Terminal::default();
    let mut tui = Tui::default(&terminal, &mut store);
    tui.run();
}
