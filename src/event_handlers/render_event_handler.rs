use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{
    app::App,
    render::drawing_context::DrawingContext,
    window_manager::event_handler::EventHandler,
};

pub struct RenderEventHandler;

impl RenderEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RenderEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for RenderEventHandler {
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        if let Err(e) = app.scene.draw_all(drawing_context) {
            println!("Failed to draw scene: {:?}", e);
        }
    }

    fn on_destroy(&mut self, _app: &mut App) {}

    fn on_resize(&mut self, _app: &mut App, _width: i32, _height: i32) {}

    fn handle_message(&mut self, _app: &mut App, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> Option<isize> {
        None
    }
}
