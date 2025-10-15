//! # Default Input Handler
//!
//! This module provides a composite event handler that bundles the most common
//! input and rendering handlers into a single, convenient unit.

use crate::core::prelude::*;
use crate::core::event::handlers::{
    input_handler::{KeyboardInputHandler, MouseInputHandler},
    render_event_handler::RenderEventHandler,
};

/// A composite [`EventHandler`] that combines the default rendering and input handlers.
///
/// This struct is a convenient shorthand for setting up the most common event
/// handlers required for a typical application:
///
/// - [`RenderEventHandler`]: Handles `Paint` events to draw the scene.
/// - [`KeyboardInputHandler`]: Tracks the state of pressed keys and keyboard modifiers.
/// - [`MouseInputHandler`]: Tracks the mouse cursor's position and button states.
///
/// By using this handler, an application can get basic rendering and input
/// state management with minimal setup. It is typically added to a [`RootEventHandler`].
pub struct DefaultInputHandler<T> {
    render_handler: RenderEventHandler<T>,
    keyboard_handler: KeyboardInputHandler,
    mouse_handler: MouseInputHandler,
}

impl<T: HasScene + HasInputContext> Default for DefaultInputHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HasScene + HasInputContext> DefaultInputHandler<T> {
    /// Creates a new `DefaultInputHandler`.
    pub fn new() -> Self {
        Self {
            render_handler: RenderEventHandler::<T>::new(),
            keyboard_handler: KeyboardInputHandler::default(),
            mouse_handler: MouseInputHandler,
        }
    }
}

impl<T: HasScene + HasInputContext> EventHandler<T> for DefaultInputHandler<T> {
    /// Dispatches the event to the child handlers.
    ///
    /// The event is passed to the `RenderEventHandler`, `KeyboardInputHandler`,
    /// and `MouseInputHandler` in sequence.
    fn on_event(&mut self, state: &mut T, event: &Event, renderer: &mut dyn Renderer) -> EventResult {
        self.render_handler.on_event(state, event, renderer);
        self.keyboard_handler.on_event(state, event, renderer);
        self.mouse_handler.on_event(state, event, renderer);
        EventResult::NotConsumed
    }
}
