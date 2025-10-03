use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;
use crate::core::event::event_handler::EventHandler;

/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct acts as the root of the event handling hierarchy. It holds a collection of
/// other `EventHandler` implementors and forwards events to them in the order they were added.
/// This design allows for a clean separation of concerns, where different aspects of event
/// handling (e.g., rendering, UI logic, logging) can be managed by separate, reusable components.
///
/// The `RootEventHandler` is the top-level handler that is passed to the `WindowBuilder` when
/// creating a window.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::event::event_handler::EventHandler;
/// use my_gui::core::event::handlers::root_event_handler::RootEventHandler;
/// use my_gui::core::event::handlers::render_event_handler::RenderEventHandler;
/// use my_gui::core::event::Event;
///
/// // Define application state
/// #[derive(Default)]
/// struct MyApp;
///
/// // Create a custom handler for logging events
/// struct LoggingEventHandler;
/// impl EventHandler<MyApp> for LoggingEventHandler {
///     fn on_event(&mut self, _app: &mut MyApp, event: &Event, _renderer: &mut dyn my_gui::core::backend::renderer::Renderer) {
///         println!("Event received: {:?}", event);
///     }
/// }
///
/// // Create a root handler and compose other handlers
/// let mut root_handler = RootEventHandler::new();
/// root_handler.add_handler(Box::new(RenderEventHandler::new())); // For drawing
/// root_handler.add_handler(Box::new(LoggingEventHandler));   // For logging
///
/// // This root_handler would then be passed to the WindowBuilder.
/// ```
pub struct RootEventHandler<T> {
    handlers: Vec<Box<dyn EventHandler<T>>>,
}

impl<T> RootEventHandler<T> {
    /// Creates a new, empty `RootEventHandler`.
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    /// Adds a new `EventHandler` to the collection.
    ///
    /// The provided handler will be boxed and added to the list of handlers. Events
    /// will be delegated to this handler after all previously added handlers have
    /// processed the event.
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
    /// on each one in the order they were added.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the application's state.
    /// - `event`: A reference to the event being processed.
    /// - `renderer`: A mutable reference to the window's renderer.
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) {
        for handler in &mut self.handlers {
            handler.on_event(app, event, renderer);
        }
    }
}