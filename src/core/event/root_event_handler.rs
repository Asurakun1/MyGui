use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::App, core::render::drawing_context::DrawingContext};
use super::event_handler::EventHandler;
use super::key_id::KeyId;

/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct acts as the root of the event handling hierarchy. It holds a collection of
/// other `EventHandler` implementors and forwards events to them. This design allows for
/// a clean separation of concerns, where different aspects of event handling (e.g., rendering,
/// input) can be managed by separate structs.
pub struct RootEventHandler {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl Default for RootEventHandler {
    fn default() -> Self {
        Self { handlers: Vec::new() }
    }
}

impl EventHandler for RootEventHandler {
    /// Delegates the `on_paint` call to all registered handlers.
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        for handler in &mut self.handlers {
            handler.on_paint(app, drawing_context);
        }
    }

    /// Delegates the `on_destroy` call to all registered handlers.
    fn on_destroy(&mut self, app: &mut App) {
        for handler in &mut self.handlers {
            handler.on_destroy(app);
        }
    }

    /// Delegates the `on_resize` call to all registered handlers.
    fn on_resize(&mut self, app: &mut App, width: i32, height: i32) {
        for handler in &mut self.handlers {
            handler.on_resize(app, width, height);
        }
    }

    /// Delegates the `on_mouse_move` call to all registered handlers.
    fn on_mouse_move(&mut self, app: &mut App, x: i32, y: i32) {
        for handler in &mut self.handlers {
            handler.on_mouse_move(app, x, y);
        }
    }

    /// Delegates the `on_lbutton_down` call to all registered handlers.
    fn on_lbutton_down(&mut self, app: &mut App, x: i32, y: i32) {
        for handler in &mut self.handlers {
            handler.on_lbutton_down(app, x, y);
        }
    }

    /// Delegates the `on_lbutton_up` call to all registered handlers.
    fn on_lbutton_up(&mut self, app: &mut App, x: i32, y: i32) {
        for handler in &mut self.handlers {
            handler.on_lbutton_up(app, x, y);
        }
    }

    /// Delegates the `on_key_down` call to all registered handlers.
    fn on_key_down(&mut self, app: &mut App, key: KeyId) {
        for handler in &mut self.handlers {
            handler.on_key_down(app, key);
        }
    }

    /// Delegates the `on_key_up` call to all registered handlers.
    fn on_key_up(&mut self, app: &mut App, key: KeyId) {
        for handler in &mut self.handlers {
            handler.on_key_up(app, key);
        }
    }

    /// Delegates the `handle_message` call to all registered handlers.
    ///
    /// It returns the result from the first handler that returns `Some`.
    fn handle_message(
        &mut self,
        app: &mut App,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        self.handlers
            .iter_mut()
            .find_map(|handler| handler.handle_message(app, msg, wparam, lparam))
    }
}