//! # Window Backend Trait
//!
//! This module defines the `WindowBackend` trait, which serves as the core
//! abstraction for creating and managing platform-specific windows.

use crate::core::event::event_handler::EventHandler;

/// Defines the generic interface for a platform-specific window implementation.
///
/// This trait abstracts the underlying details of how a window is created and how
/// its message loop is run. The [`WindowBuilder`] is responsible for creating a
/// concrete implementation of this trait (e.g., `Win32Window`) for the target
/// operating system.
///
/// By returning a `Box<dyn WindowBackend<T, E>>`, the framework allows the
/// application to interact with the window in a platform-agnostic manner.
///
/// # Type Parameters
///
/// * `T`: The application's state type, which is managed by the window.
/// * `E`: The application's root event handler, which must implement `EventHandler<T>`.
pub trait WindowBackend<T, E: EventHandler<T>> {
    /// Runs the window's main message loop.
    ///
    /// This method starts the application's event loop, which is responsible for
    /// listening for and dispatching operating system messages (e.g., user input,
    /// paint requests, etc.). This function will block the current thread until
    /// the window is closed and the message loop terminates.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` which will be `Ok(())` if the message loop exits
    /// gracefully, or an error if one occurs during the process.
    fn run(&self) -> anyhow::Result<()>;
}