use crate::core::render::drawing_context::DrawingContext;
use super::event_handler::EventHandler;
use crate::core::render::scene::HasScene;
use std::marker::PhantomData;

/// An event handler that is responsible for rendering the application's scene.
///
/// This handler implements the `on_paint` method to draw the contents of the
/// `App`'s `Scene` to the window.
pub struct RenderEventHandler<T> {
    _phantom: PhantomData<T>,
}

impl<T> RenderEventHandler<T> {
    /// Creates a new `RenderEventHandler`.
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<T> Default for RenderEventHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

use windows::Win32::Graphics::Direct2D::{Common::D2D1_COLOR_F, ID2D1RenderTarget};

impl<T: HasScene> EventHandler<T> for RenderEventHandler<T> {
    /// Handles the `WM_PAINT` message by clearing the render target and drawing the scene.
    ///
    /// # Safety
    ///
    /// This method contains `unsafe` blocks for calling Direct2D methods.
    /// The caller must ensure that the `drawing_context` contains valid Direct2D
    /// resources.
    fn on_paint(&mut self, app: &mut T, drawing_context: &DrawingContext) {
        unsafe {
            drawing_context.render_target.BeginDraw();
            let rt: &ID2D1RenderTarget = drawing_context.render_target;
            rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }));

            if let Err(e) = app.scene().draw_all(drawing_context) {
                println!("Failed to draw scene: {:?}", e);
            }

            if let Err(e) = drawing_context.render_target.EndDraw(None, None) {
                println!("EndDraw failed: {:?}", e);
            }
        }
    }
}
