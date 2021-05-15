mod tui;
use tui::Tui;
mod terminal;
use terminal::Terminal;
mod task;

fn main() {
    let terminal = Terminal::default();
    let mut tui = Tui::default(&terminal);
    tui.run();
}
