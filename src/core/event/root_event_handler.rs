use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;
use super::event_handler::EventHandler;

/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct acts as the root of the event handling hierarchy. It holds a collection of
/// other `EventHandler` implementors and forwards events to them. This design allows for
/// a clean separation of concerns, where different aspects of event handling (e.g., rendering,
/// input) can be managed by separate structs.
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
    /// The provided handler will be boxed and added to the list of handlers to which
    /// this `RootEventHandler` delegates.
    pub fn add_handler(&mut self, handler: Box<dyn EventHandler<T>>) {
        self.handlers.push(handler);
    }
}

impl<T> Default for RootEventHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EventHandler<T> for RootEventHandler<T> {
    /// Delegates the `on_event` call to all registered handlers.
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) {
        for handler in &mut self.handlers {
            handler.on_event(app, event, renderer);
        }
    }
}