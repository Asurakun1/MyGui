//! # The EventHandler Trait
//!
//! This module defines the `EventHandler` trait, the fundamental building block
//! of the event processing system in the framework.

use crate::core::prelude::*;

/// A generic trait for handling window and input events.
///
/// `EventHandler` provides a single method, `on_event`, which is called by the
/// main event loop whenever a new [`crate::prelude::Event`] occurs. This design decouples the
/// application logic from the low-level, platform-specific message processing.
///
/// This trait is generic over a type `T`, which represents the application's
/// shared state. This allows any event handler to access and modify the
/// application state in a type-safe manner, providing the context needed to
/// react to events.
///
/// ## Composition
///
/// Handlers are designed to be composed. The [`crate::prelude::RootEventHandler`] maintains a
/// list of child handlers and dispatches events to each of them in sequence.
/// This promotes a modular architecture where different handlers can manage
/// separate concerns (e.g., rendering, input tracking, UI logic).
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::event::{Event, event_handler::EventHandler};
/// use my_gui::core::backend::renderer::Renderer;
///
/// // 1. Define your application's state.
/// struct MyApp {
///     click_count: i32,
/// }
///
/// // 2. Create a custom event handler struct.
/// struct AppLogicHandler;
///
/// // 3. Implement the EventHandler trait.
/// impl EventHandler<MyApp> for AppLogicHandler {
///     fn on_event(&mut self, app: &mut MyApp, event: &Event, renderer: &mut dyn Renderer) {
///         match event {
///             Event::MouseDown(_) => {
///                 app.click_count += 1;
///                 println!("Mouse clicked! Total clicks: {}", app.click_count);
///             }
///             Event::WindowClose => {
///                 println!("Window close requested. Final count: {}", app.click_count);
///             }
///             _ => { /* Ignore other events */ }
///         }
///     }
/// }
/// ```
pub trait EventHandler<T> {
    /// Processes a new event received from the window.
    ///
    /// This method is the central entry point for all event processing. It is
    /// called for every [`crate::prelude::Event`] that the window receives. The default
    /// implementation is a no-op, allowing implementors to only handle the
    /// events they are interested in.
    ///
    /// The method now returns a `bool`:
    /// - `true`: The event was consumed by this handler, and propagation should stop.
    /// - `false`: The event was not consumed, and propagation should continue to the next handler.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the application's state object (`T`).
    ///   This provides the context needed to react to the event.
    /// - `event`: A reference to the [`crate::prelude::Event`] that occurred. A `match` statement
    ///   is typically used here to dispatch to event-specific logic.
    /// - `renderer`: A mutable reference to the window's [`crate::prelude::Renderer`]. This can be
    ///   used for immediate drawing operations, though rendering is typically
    ///   deferred to the [`crate::core::event::handlers::render_event_handler::RenderEventHandler`] in response to a `Paint` event.
    fn on_event(&mut self, _app: &mut T, _event: &Event, _renderer: &mut dyn Renderer) -> bool { false }

    /// Handles an error that occurred in the event loop.
    ///
    /// This method is called when an error occurs in the `wndproc` function.
    /// The default implementation logs the error.
    ///
    /// # Parameters
    ///
    /// - `error`: The error that occurred.
    fn on_error(&mut self, error: &anyhow::Error) {
        log::error!("An error occurred in the event loop: {:?}", error);
    }
}