//! # Window Backend Trait
//!
//! This module defines the `WindowBackend` trait, which serves as the core
//! abstraction for creating and managing platform-specific windows.

use crate::core::event::event_handler::EventHandler;

/// Defines the interface for a platform-specific window implementation.
///
/// This trait abstracts the underlying native windowing system, providing a
/// consistent, high-level API for the rest of the framework. It is responsible
/// for handling the window's lifecycle, including its creation, message loop,
/// and destruction.
///
/// Each supported platform (e.g., Windows, macOS, Linux) must provide a concrete
/// implementation of this trait.
///
/// # Type Parameters
///
/// - `T`: The application's state type, which is passed to the event handler.
/// - `E`: The root event handler for the window, which must implement `EventHandler<T>`.
pub trait WindowBackend<T, E: EventHandler<T>> {
    /// Runs the window's main message loop.
    ///
    /// This method takes ownership of the window backend (`self: Box<Self>`)
    /// because it is the entry point for the window's entire lifecycle. It begins
    /// listening for and dispatching native OS events, translating them into
    /// platform-agnostic `Event`s that are passed to the `EventHandler`.
    ///
    /// This function will block the current thread until the window is closed
    /// and the message loop terminates.
    ///
    /// # Errors
    ///
    /// Returns an error if the message loop fails to start or encounters an
    /// unrecoverable error during execution.
    fn run(self: Box<Self>) -> anyhow::Result<()>;
}