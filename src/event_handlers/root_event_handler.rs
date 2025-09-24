
use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{
    app::App,
    render::drawing_context::DrawingContext,
    window_manager::event_handler::EventHandler,
};

use super::render_event_handler::RenderEventHandler;

pub struct RootEventHandler {
    render_event_handler: RenderEventHandler,
}

impl RootEventHandler {
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
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        self.render_event_handler.on_paint(app, drawing_context);
    }

    fn on_destroy(&mut self, app: &mut App) {
        self.render_event_handler.on_destroy(app);
        println!("Window destroyed");
    }

    fn on_resize(&mut self, app: &mut App, width: i32, height: i32) {
        self.render_event_handler.on_resize(app, width, height);
        println!("Window resized to {}x{}", width, height);
    }

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
