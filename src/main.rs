mod gl;
mod window;

use window::LmmpWindow;

fn main() {
    let window = LmmpWindow::new();
    window.run();
}
