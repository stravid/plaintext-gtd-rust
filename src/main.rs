mod tui;
use tui::Tui;
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
    let tui = Tui {};
    tui.run();
}
