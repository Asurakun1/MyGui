use super::event_handler::EventHandler;
use crate::core::backend::renderer::Renderer;
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

impl<T: HasScene> EventHandler<T> for RenderEventHandler<T> {
    /// Handles the `WM_PAINT` message by clearing the render target and drawing the scene.
    ///
    /// # Safety
    ///
    /// This method contains `unsafe` blocks for calling Direct2D methods.
    /// The caller must ensure that the `renderer` contains valid Direct2D
    /// resources.
    fn on_paint(&mut self, app: &mut T, renderer: &mut dyn Renderer) {
        renderer.begin_draw();
        renderer.clear(0.0, 0.0, 0.0, 1.0);

        if let Err(e) = app.scene().draw_all(renderer) {
            println!("Failed to draw scene: {:?}", e);
        }

        if let Err(e) = renderer.end_draw() {
            println!("EndDraw failed: {:?}", e);
        }
    }
}
