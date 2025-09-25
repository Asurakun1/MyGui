use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::app::App, event::event_handler::EventHandler, render::drawing_context::DrawingContext};

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

use windows::Win32::Graphics::Direct2D::{Common::D2D1_COLOR_F, ID2D1RenderTarget};

use crate::event::key_id::KeyId;

impl EventHandler for RenderEventHandler {
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        unsafe {
            drawing_context.render_target.BeginDraw();
            let rt: &ID2D1RenderTarget = drawing_context.render_target;
            rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }));

            if let Err(e) = app.scene.draw_all(drawing_context) {
                println!("Failed to draw scene: {:?}", e);
            }

            if let Err(e) = drawing_context.render_target.EndDraw(None, None) {
                println!("EndDraw failed: {:?}", e);
            }
        }
    }

    fn on_destroy(&mut self, _app: &mut App) {}

    fn on_resize(&mut self, _app: &mut App, _width: i32, _height: i32) {}

    fn on_mouse_move(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    fn on_lbutton_down(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    fn on_lbutton_up(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    fn on_key_down(&mut self, _app: &mut App, _key: KeyId) {}

    fn on_key_up(&mut self, _app: &mut App, _key: KeyId) {}

    fn handle_message(
        &mut self,
        _app: &mut App,
        _msg: u32,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Option<isize> {
        None
    }
}
