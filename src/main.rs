use windows::core::*;

mod window_manager;
mod render;
use window_manager::config::{WINDOW_TITLE, WINDOW_CLASS_NAME};

fn main() -> Result<()> {
    let window = window_manager::window::Window::new(WINDOW_TITLE, WINDOW_CLASS_NAME)?;
    let result = window.message_loop();
    std::mem::forget(window);
    result
}