//! # Root Event Handler
//!
//! This module provides the `RootEventHandler`, which acts as the primary
//! dispatcher in the event handling system.

use crate::core::{
    backend::renderer::Renderer,
    event::{event_handler::EventHandler, Event},
};

/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct acts as the root of the event handling hierarchy. It maintains a
/// collection of child `EventHandler` implementors and forwards events to them
/// in the order they were added. This "composition over inheritance" design
/// allows for a clean separation of concerns, where different aspects of event
/// handling (e.g., rendering, input tracking, UI logic) can be managed by
/// separate, reusable components.
///
/// The `RootEventHandler` is the top-level handler that is passed to the
/// `WindowBuilder` when creating a window.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::event::handlers::{
///     root_event_handler::RootEventHandler,
///     render_event_handler::RenderEventHandler,
///     // other handlers...
/// };
/// use my_gui::core::event::{Event, event_handler::EventHandler};
///
/// // Define application state
/// #[derive(Default)]
/// struct MyApp;
///
/// // Create a custom handler for application logic
/// struct AppLogicHandler;
/// impl EventHandler<MyApp> for AppLogicHandler {
///     /* ... */
/// }
///
/// // Create a root handler and compose the built-in and custom handlers
/// let mut root_handler = RootEventHandler::new();
/// root_handler.add_handler(Box::new(RenderEventHandler::default())); // Handles drawing
/// // root_handler.add_handler(Box::new(KeyboardInputHandler::default())); // Handles keyboard state
/// // root_handler.add_handler(Box::new(MouseInputHandler));      // Handles mouse state
/// root_handler.add_handler(Box::new(AppLogicHandler));         // Handles custom logic
///
/// // This `root_handler` would then be passed to the `WindowBuilder`.
/// ```
pub struct RootEventHandler<T> {
    handlers: Vec<Box<dyn EventHandler<T>>>,
}

impl<T> RootEventHandler<T> {
    /// Creates a new, empty `RootEventHandler`.
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    /// Adds a new [`EventHandler`] to the collection.
    ///
    /// The provided handler will be boxed and added to the end of the delegation
    /// list. Events will be propagated to this handler after all previously
    /// added handlers have processed the event.
    ///
    /// # Arguments
    ///
    /// * `handler` - A `Box<dyn EventHandler<T>>` to be added to the delegation list.
    pub fn add_handler(&mut self, handler: Box<dyn EventHandler<T>>) {
        self.handlers.push(handler);
    }
}

impl<T> Default for RootEventHandler<T> {
    /// Creates a default `RootEventHandler`, which is equivalent to `RootEventHandler::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EventHandler<T> for RootEventHandler<T> {
    /// Delegates the incoming event to all registered child handlers.
    ///
    /// This method iterates through its collection of handlers and calls `on_event`
    /// on each one in the order they were added, allowing each handler to process
    /// the event.
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) {
        for handler in &mut self.handlers {
            handler.on_event(app, event, renderer);
        }
    }
}