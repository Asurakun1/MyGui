use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{app::app::App, event::event_handler::EventHandler, render::drawing_context::DrawingContext};

/// An event handler that is responsible for rendering the application's scene.
pub struct RenderEventHandler;

impl RenderEventHandler {
    /// Creates a new `RenderEventHandler`.
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
    /// Handles the `WM_PAINT` message and renders the scene.
    fn on_paint(&mut self, app: &mut App, drawing_context: &DrawingContext) {
        unsafe {
            drawing_context.render_target.BeginDraw();
            let rt: &ID2D1RenderTarget = drawing_context.render_target;
            rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }));

            if let Err(e) = app.scene.draw_all(drawing_context) {
                println!("Failed to draw scene: {:?}", e);
            }

            if let Err(e) = drawing_context.render_target.EndDraw(None, None) {
                println!("EndDraw failed: {:?}", e);
            }
        }
    }

    /// Handles the `WM_DESTROY` message.
    fn on_destroy(&mut self, _app: &mut App) {}

    /// Handles the `WM_SIZE` message.
    fn on_resize(&mut self, _app: &mut App, _width: i32, _height: i32) {}

    /// Handles mouse move events.
    fn on_mouse_move(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    /// Handles left mouse button down events.
    fn on_lbutton_down(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    /// Handles left mouse button up events.
    fn on_lbutton_up(&mut self, _app: &mut App, _x: i32, _y: i32) {}

    /// Handles key down events.
    fn on_key_down(&mut self, _app: &mut App, _key: KeyId) {}

    /// Handles key up events.
    fn on_key_up(&mut self, _app: &mut App, _key: KeyId) {}

    /// Handles any other window messages.
    fn handle_message(&mut self, _app: &mut App, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> Option<isize> {
        None
    }
}
