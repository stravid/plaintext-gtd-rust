mod tui;
use tui::Tui;
mod terminal;
use terminal::Terminal;
/*
Tui
Store
Keyboard
Terminal
TaskListScreen
TaskFormScreen
Task

Next step is to move everything in `main` to a file called `tui.rs`.
*/
fn main() {
    let terminal = Terminal::default();
    let mut tui = Tui::default(&terminal);
    tui.run();
}
