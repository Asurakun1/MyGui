use crate::core::event::event_handler::EventHandler;
use windows::core::Result;

/// A trait for platform-specific window implementations.
///
/// This trait defines the common interface for creating and managing a window
/// on a specific platform.
pub trait WindowBackend<T, E: EventHandler<T>> {
    /// Runs the message loop for the window.
    fn run(&self) -> Result<()>;
}
