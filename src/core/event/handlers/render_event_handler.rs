//! # Render Event Handler
//!
//! This module provides the `RenderEventHandler`, a specialized handler
//! responsible for orchestrating the drawing of the application's scene.

use crate::core::{
    backend::renderer::Renderer,
    event::{event_handler::EventHandler, Event},
    render::{color::Color, scene::HasScene},
};
use std::marker::PhantomData;

/// An [`EventHandler`] responsible for rendering the application's scene graph.
///
/// This handler specifically listens for the [`Event::Paint`] event. When this
/// event is received, it orchestrates the entire drawing process for a single frame:
///
/// 1. It calls `begin_draw()` on the [`Renderer`].
/// 2. It clears the render target with a solid background color.
/// 3. It traverses the application's `Scene` and calls the `draw` method on
///    every `Drawable` object.
/// 4. It calls `end_draw()` on the [`Renderer`] to present the final frame.
///
/// For this handler to function, the application's state struct (`T`) must
/// implement the `HasScene` trait, which provides access to the `Scene` that
/// needs to be rendered.
///
/// This handler is essential for any application that displays graphics and should
/// be added to the `RootEventHandler`.
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

            // Clear the background to a default color.
            renderer.clear(&Color::BLACK);

            // Draw all objects in the scene graph.
            if let Err(e) = app.scene().draw_all(renderer) {
                // In a real application, this should be logged more robustly.
                log::error!("Failed to draw scene: {:?}", e);
            }

            // Finalize and present the frame.
            if let Err(e) = renderer.end_draw() {
                log::error!("EndDraw failed: {:?}", e);
            }
        }
    }
}