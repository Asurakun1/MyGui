use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;

/// Defines the interface for handling window events.
///
/// This trait provides a structured way to respond to common window messages.
/// An `EventHandler` is associated with a `Window` and its methods are called
/// from the `wndproc` function when the corresponding messages are received.
///
/// Implementors of this trait can be composed to create more complex event
/// handling logic (see `RootEventHandler`).
pub trait EventHandler<T> {
    /// Called when a new event is received.
    fn on_event(&mut self, _app: &mut T, _event: &Event, _renderer: &mut dyn Renderer) {}
}
