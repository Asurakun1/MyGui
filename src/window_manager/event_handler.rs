use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::App, render::drawing_context::DrawingContext};

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
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext);

    /// Called when the window is being destroyed (in response to `WM_DESTROY`).
    fn on_destroy(&mut self, app: &mut App);

    /// Called when the window is resized (in response to `WM_SIZE`).
    fn on_resize(&mut self, app: &mut App, width: i32, height: i32);

    /// A catch-all method for handling any other window messages.
    ///
    /// If this method handles the message, it should return `Some(result)`.
    /// If it does not handle the message, it should return `None`, allowing
    /// for further processing or default handling.
    fn handle_message(&mut self, app: &mut App, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize>;
}
