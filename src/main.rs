mod buffer;
mod character;
mod cursor;
mod editor;
mod line;
mod math;
mod renderer;
mod utils;
mod window;

use editor::Editor;

fn main() {
    let mut editor = Editor::new();
    editor.start();
}
