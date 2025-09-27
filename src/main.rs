//! # my_gui Framework
//!
//! This project is a custom GUI framework for Windows built using the `windows` crate.
//! It serves as a learning exercise to understand the fundamentals of Windows GUI programming,
//! including window creation, message handling, and Direct2D rendering.
//!
//! ## Architecture
//!
//! The application is structured into several key modules:
//!
//! - **`app`**: Contains the central application state (`App` struct), which holds data
//!   like the scene to be rendered.
//! - **`window_manager`**: Handles the creation and management of the application window,
//!   including the message loop.
//! - **`render`**: Provides abstractions for Direct2D rendering, including a `Scene`,
//!   `Drawable` objects, and a `DrawingContext`.
//! - **`event_handlers`**: Defines a composable system for handling window events (like
//!   `WM_PAINT` and `WM_DESTROY`).
//!
//! The `main` function initializes these components, creates the main window, and starts
//! the message loop. The `Window` instance is intentionally "leaked" using `std::mem::forget`
//! because its lifetime is managed by the Windows API through the `wndproc` callback.

use my_gui::*;

use windows::{
    core::*,
};

use window::WindowBuilder;
use event::root_event_handler::RootEventHandler;
use event::render_event_handler::RenderEventHandler;
use app::App;


fn main() -> Result<()> {
    let app = App::new();
    let mut event_handler = RootEventHandler::new();
    event_handler.add_handler(Box::new(RenderEventHandler::new()));
    let window = WindowBuilder::new().build(event_handler, app)?;
    let result = window.message_loop();

    // The `Window` is allocated on the heap, and a raw pointer to it is stored in the
    // Win32 window's user data. The `wndproc` is responsible for freeing this memory
    // when the window is destroyed (during `WM_NCDESTROY`).
    //
    // We call `std::mem::forget` to prevent Rust from dropping the `Box<Window>` when it
    // goes out of scope here. If we allowed the drop to happen, the memory would be
    // deallocated, but the `HWND` and its `wndproc` would still exist, holding a
    // dangling pointer. This would lead to a use-after-free bug when the next window
    // message is processed.
    std::mem::forget(window);

    result
}