
use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{
    app::App,
    render::drawing_context::DrawingContext,
    window_manager::event_handler::EventHandler,
};

use super::render_event_handler::RenderEventHandler;

/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct acts as the root of the event handling hierarchy. It holds instances of
/// other `EventHandler` implementors and forwards events to them. This design allows for
/// a clean separation of concerns, where different aspects of event handling (e.g., rendering,
/// input) can be managed by separate structs.
pub struct RootEventHandler {
    render_event_handler: RenderEventHandler,
}

impl RootEventHandler {
    /// Creates a new `RootEventHandler` and initializes its child handlers.
    pub fn new() -> Self {
        Self {
            render_event_handler: RenderEventHandler::new(),
        }
    }
}

impl Default for RootEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for RootEventHandler {
    /// Delegates the `on_paint` event to the appropriate child handler.
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        self.render_event_handler.on_paint(app, drawing_context);
    }

    /// Delegates the `on_destroy` event.
    fn on_destroy(&mut self, app: &mut App) {
        self.render_event_handler.on_destroy(app);
        println!("Window destroyed");
    }

    /// Delegates the `on_resize` event.
    fn on_resize(&mut self, app: &mut App, width: i32, height: i32) {
        self.render_event_handler.on_resize(app, width, height);
        println!("Window resized to {}x{}", width, height);
    }

    /// Delegates general message handling.
    fn handle_message(
        &mut self,
        app: &mut App,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        self.render_event_handler
            .handle_message(app, msg, wparam, lparam)
    }
}
