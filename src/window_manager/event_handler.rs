use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::App, render::drawing_context::DrawingContext};

/// A trait for handling window events.
///
/// This trait defines the interface for an object that can handle events from a `Window`.
/// It allows for a separation of concerns between the windowing code and the application logic.
pub trait EventHandler {
    /// Called when the window receives a `WM_PAINT` message.
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext);
    /// Called when the window receives a `WM_DESTROY` message.
    fn on_destroy(&mut self, app: &mut App);
    /// Called when the window receives a `WM_SIZE` message.
    fn on_resize(&mut self, app: &mut App, width: i32, height: i32);
    /// Called for any other window message.
    /// If the function returns `Some(result)`, the window procedure will return that value.
    /// If it returns `None`, the message will be passed to `DefWindowProcW`.
    fn handle_message(&mut self, app: &mut App, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize>;
}
