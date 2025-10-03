//! # Event Handler Trait
//!
//! This module defines the `EventHandler` trait, which is the core of the event
//! processing system in the `my_gui` framework.

use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;

/// A trait for handling window and input events.
///
/// `EventHandler` provides a single method, `on_event`, which is called by the
/// window's message loop whenever a new event occurs. This allows application
/// logic to be decoupled from the low-level message processing.
///
/// This trait is generic over a type `T`, which represents the application's state.
/// This allows the event handler to access and modify the application state in
/// response to events.
///
/// ## Composition
///
/// Handlers can be composed using the `RootEventHandler`, which maintains a list
/// of child handlers. When an event is received, the `RootEventHandler` dispatches
/// it to all of its children, allowing for a modular and extensible architecture.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::event::{Event, event_handler::EventHandler};
/// use my_gui::core::backend::renderer::Renderer;
///
/// // 1. Define your application state.
/// struct MyApp {
///     counter: i32,
/// }
///
/// // 2. Create a custom event handler.
/// struct MyEventHandler;
///
/// // 3. Implement the EventHandler trait for your handler.
/// impl EventHandler<MyApp> for MyEventHandler {
///     fn on_event(&mut self, app: &mut MyApp, event: &Event, renderer: &mut dyn Renderer) {
///         match event {
///             Event::MouseDown(_) => {
///                 app.counter += 1;
///                 println!("Mouse down! Counter is now: {}", app.counter);
///             }
///             Event::WindowClose => {
///                 println!("Window close requested. Final count: {}", app.counter);
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
pub trait EventHandler<T> {
    /// Called when a new event is received from the window.
    ///
    /// This method is the central point for processing all window and input events.
    /// It is called for every `Event` that the window processes. The default
    /// implementation does nothing, so you only need to implement logic for the
    /// events you are interested in.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the application's state object (`T`). This
    ///   allows the handler to read and modify the application state.
    /// - `event`: A reference to the `Event` that occurred. Use a `match` statement
    ///   on this enum to handle different types of events.
    /// - `renderer`: A mutable reference to the window's `Renderer`. This can be
    ///   used to perform drawing operations, although rendering is typically
    ///   handled by the `RenderEventHandler` in response to a `Paint` event.
    fn on_event(&mut self, _app: &mut T, _event: &Event, _renderer: &mut dyn Renderer) {}
}