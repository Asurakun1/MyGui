
use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::app::App, event::event_handler::EventHandler, render::drawing_context::DrawingContext};



/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct acts as the root of the event handling hierarchy. It holds instances of
/// other `EventHandler` implementors and forwards events to them. This design allows for
/// a clean separation of concerns, where different aspects of event handling (e.g., rendering,
/// input) can be managed by separate structs.
pub struct RootEventHandler {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl RootEventHandler {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn add_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }
}

impl Default for RootEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

use crate::event::key_id::KeyId;

impl EventHandler for RootEventHandler {
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        for handler in &mut self.handlers {
            handler.on_paint(app, drawing_context);
        }
    }

    fn on_destroy(&mut self, app: &mut App) {
        for handler in &mut self.handlers {
            handler.on_destroy(app);
        }
    }

    fn on_resize(&mut self, app: &mut App, width: i32, height: i32) {
        for handler in &mut self.handlers {
            handler.on_resize(app, width, height);
        }
    }

    fn on_mouse_move(&mut self, app: &mut App, x: i32, y: i32) {
        for handler in &mut self.handlers {
            handler.on_mouse_move(app, x, y);
        }
    }

    fn on_lbutton_down(&mut self, app: &mut App, x: i32, y: i32) {
        for handler in &mut self.handlers {
            handler.on_lbutton_down(app, x, y);
        }
    }

    fn on_lbutton_up(&mut self, app: &mut App, x: i32, y: i32) {
        for handler in &mut self.handlers {
            handler.on_lbutton_up(app, x, y);
        }
    }

    fn on_key_down(&mut self, app: &mut App, key: KeyId) {
        for handler in &mut self.handlers {
            handler.on_key_down(app, key);
        }
    }

    fn on_key_up(&mut self, app: &mut App, key: KeyId) {
        for handler in &mut self.handlers {
            handler.on_key_up(app, key);
        }
    }

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
