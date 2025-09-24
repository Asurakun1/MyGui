
use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{
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

impl EventHandler for RootEventHandler {
    fn on_paint(&mut self, drawing_context: &DrawingContext) {
        self.render_event_handler.on_paint(drawing_context);
    }

    fn on_destroy(&mut self) {
        println!("Window destroyed");
    }

    fn on_resize(&mut self, width: i32, height: i32) {
        println!("Window resized to {}x{}", width, height);
    }

    fn handle_message(&mut self, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> Option<isize> {
        None
    }
}
