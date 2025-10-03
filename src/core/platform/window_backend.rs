//! # Window Backend Trait
//!
//! This module defines the `WindowBackend` trait, which is the core abstraction
//! for platform-specific window implementations.

use crate::core::event::event_handler::EventHandler;

/// A trait that defines the interface for a platform-specific window.
///
/// This trait abstracts the underlying implementation of a window, such as its
/// creation, message loop, and event dispatching. The `WindowBuilder` is responsible
/// for creating a concrete implementation of this trait (e.g., `Win32Window`)
/// based on the target operating system.
///
/// The public-facing API returns a `Box<dyn WindowBackend<T, E>>`, allowing the
/// application to interact with the window without being coupled to the specific
/// platform.
///
/// # Type Parameters
///
/// * `T`: The application's state type.
/// * `E`: The application's root event handler type, which must implement `EventHandler<T>`.
pub trait WindowBackend<T, E: EventHandler<T>> {
    /// Runs the window's main message loop.
    ///
    /// This method starts the application's main loop, which listens for and
    /// dispatches operating system messages (e.g., user input, paint requests).
    /// This function will block until the window is closed and the message loop
    /// terminates.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` which will be `Ok(())` if the message loop terminates
    /// gracefully, or contain an error if one occurs during shutdown.
    fn run(&self) -> anyhow::Result<()>;
}