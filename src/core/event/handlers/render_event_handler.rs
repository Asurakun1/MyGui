use crate::core::event::event_handler::EventHandler;
use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;
use crate::core::render::scene::HasScene;
use crate::core::render::color::Color;
use std::marker::PhantomData;

/// An `EventHandler` responsible for rendering the application's scene.
///
/// This handler specifically listens for the `Event::Paint` event. When this event
/// is received, it orchestrates the entire drawing process for a single frame:
///
/// 1. It calls `begin_draw()` on the `Renderer`.
/// 2. It clears the render target with a solid color (currently black).
/// 3. It iterates through all `Drawable` objects in the application's `Scene`
///    and calls their respective `draw` methods.
/// 4. It calls `end_draw()` on the `Renderer` to present the final frame.
///
/// For this handler to function, the application's state struct (`T`) must
/// implement the `HasScene` trait, which provides access to the `Scene` that
/// needs to be rendered.
///
/// This handler is essential for any application that needs to display graphics
/// and should typically be one of the first handlers added to the `RootEventHandler`.
pub struct RenderEventHandler<T> {
    _phantom: PhantomData<T>,
}

impl<T> RenderEventHandler<T> {
    /// Creates a new `RenderEventHandler`.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for RenderEventHandler<T> {
    /// Creates a default `RenderEventHandler`, which is equivalent to `RenderEventHandler::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HasScene> EventHandler<T> for RenderEventHandler<T> {
    /// Handles the `Paint` event by clearing the render target and drawing the scene.
    ///
    /// This method is called for every event, but it only takes action if the
    /// event is `Event::Paint`. All other events are ignored by this handler.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the application state, which must implement `HasScene`.
    /// - `event`: The event being processed.
    /// - `renderer`: The renderer used to perform drawing operations.
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) {
        if let Event::Paint = event {
            renderer.begin_draw();
            // Clear the background to black using the framework's Color struct.
            renderer.clear(&Color::BLACK);

            // Draw all objects in the scene.
            if let Err(e) = app.scene().draw_all(renderer) {
                // In a real application, this should be logged more robustly.
                                log::error!("Failed to draw scene: {:?}", e);
            }

            // Finalize the drawing for this frame.
            if let Err(e) = renderer.end_draw() {
                            log::error!("EndDraw failed: {:?}", e);
            }
        }
    }
}