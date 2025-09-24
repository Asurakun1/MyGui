use windows::core::*;

mod window_manager;
mod render;
use window_manager::config::{WINDOW_TITLE, WINDOW_CLASS_NAME};

/// The main entry point of the application.
///
/// This function initializes the window, runs the message loop, and handles the
/// overall lifecycle of the application.
fn main() -> Result<()> {
    let window = window_manager::window::Window::new(WINDOW_TITLE, WINDOW_CLASS_NAME)?;
    let result = window.message_loop();

    // The `Window` is wrapped in a `Box` and a pointer to it is stored in the
    // window's user data. When the window is destroyed (in `WM_NCDESTROY`),
    // the pointer is retrieved and the `Box` is reconstructed and dropped,
    // ensuring that the `Window`'s resources are properly cleaned up.
    //
    // `std::mem::forget` is used here to prevent the `Box<Window>` from being
    // dropped at the end of the `main` function, as its lifetime is now tied
    // to the window itself.
    std::mem::forget(window);

    result
}