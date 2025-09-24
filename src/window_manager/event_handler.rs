use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::render::drawing_context::DrawingContext;

pub trait EventHandler {
    fn on_paint(&mut self, drawing_context: &DrawingContext);
    fn on_destroy(&mut self);
    fn on_resize(&mut self, width: i32, height: i32);
    fn handle_message(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize>;
}
