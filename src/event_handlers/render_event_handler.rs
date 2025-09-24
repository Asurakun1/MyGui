use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{
    render::{drawing_context::DrawingContext, objects::text_object::TextObject, scene::Scene},
    window_manager::{config::DISPLAY_TEXT, event_handler::EventHandler},
};

pub struct RenderEventHandler {
    scene: Scene,
}

impl RenderEventHandler {
    pub fn new() -> Self {
        let mut scene = Scene::new();
        scene.add_object(Box::new(TextObject::new(DISPLAY_TEXT, 10.0, 10.0)));
        Self { scene }
    }
}

impl EventHandler for RenderEventHandler {
    fn on_paint(&mut self, drawing_context: &DrawingContext) {
        if let Err(e) = self.scene.draw_all(drawing_context) {
            println!("Failed to draw scene: {:?}", e);
        }
    }

    fn on_destroy(&mut self) {}

    fn on_resize(&mut self, _width: i32, _height: i32) {}

    fn handle_message(&mut self, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> Option<isize> {
        None
    }
}
