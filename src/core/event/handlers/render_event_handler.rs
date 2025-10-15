//! # Render Event Handler
//!
//! This module provides the `RenderEventHandler`, a specialized handler
//! responsible for orchestrating the drawing of the application's scene.

use crate::core::prelude::*;
use std::marker::PhantomData;

/// An [`EventHandler`] that is specialized for handling the `Paint` event.
///
/// This handler is a critical component for any visible application, as it
/// connects the rendering system to the event loop. Its sole responsibility is
/// to listen for the `Paint` event and, when it receives one, to orchestrate
/// the full rendering of the application's scene.
///
/// ## Rendering Cycle
///
/// When a `Paint` event occurs, this handler performs the following sequence of operations:
///
/// 1. **`begin_draw()`**: Prepares the renderer for a new frame.
/// 2. **`clear()`**: Clears the render target with a background color (defaulting to black).
/// 3. **`scene.draw_all()`**: Traverses the application's scene graph and calls the
///    `draw` method on every [`Drawable`] object.
/// 4. **`end_draw()`**: Finalizes the frame and presents it to the window.
///
/// This handler is typically included as part of the [`DefaultInputHandler`] or
/// added directly to a [`RootEventHandler`].
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
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HasScene> EventHandler<T> for RenderEventHandler<T> {
    /// Handles the `Paint` event by drawing the application's scene.
    ///
    /// This method ignores all events except for `Event::Paint`. Upon receiving
    /// a `Paint` event, it executes the full rendering cycle for the scene.
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) -> EventResult {
        if let Event::Paint = event {
            renderer.begin_draw();
            renderer.clear(&Color::BLACK);

            if let Err(e) = app.scene_mut().draw_all(renderer) {
                log::error!("Failed to draw scene: {:?}", e);
            }

            if let Err(e) = renderer.end_draw() {
                log::error!("EndDraw failed: {:?}", e);
            }
        }
        EventResult::NotConsumed
    }
}
