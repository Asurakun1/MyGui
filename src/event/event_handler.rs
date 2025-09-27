use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::App, render::drawing_context::DrawingContext};
use crate::event::key_id::KeyId;

/// Defines the interface for handling window events.
///
/// This trait provides a structured way to respond to common window messages.
/// An `EventHandler` is associated with a `Window` and its methods are called
/// from the `wndproc` function when the corresponding messages are received.
///
/// Implementors of this trait can be composed to create more complex event
/// handling logic (see `RootEventHandler`).
pub trait EventHandler {
    /// Called when the window needs to be repainted (in response to `WM_PAINT`).
    fn on_paint(&mut self, _app: &mut App, _drawing_context: &DrawingContext) {}

    /// Called when the window is being destroyed (in response to `WM_DESTROY`).
    fn on_destroy(&mut self, _app: &mut App) {}

    /// Called when the window is resized (in response to `WM_SIZE`).
    fn on_resize(&mut self, _app: &mut App, _width: i32, _height: i32) {}

    /// Called when the mouse moves over the window client area.
    fn on_mouse_move(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    /// Called when the left mouse button is pressed.
    fn on_lbutton_down(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    /// Called when the left mouse button is released.
    fn on_lbutton_up(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    /// Called when a non-system key is pressed.
    fn on_key_down(&mut self, _app: &mut App, _key: KeyId) {}

    /// Called when a non-system key is released.
    fn on_key_up(&mut self, _app: &mut App, _key: KeyId) {}

    /// A catch-all method for handling any other window messages.
    ///
    /// If this method handles the message, it should return `Some(result)`.
    /// If it does not handle the message, it should return `None`, allowing
    /// for further processing or default handling by `DefWindowProcW`.
    fn handle_message(&mut self, _app: &mut App, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> Option<isize> {
        None
    }
}
