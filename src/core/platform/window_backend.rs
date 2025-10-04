//! # Window Backend Trait
//!
//! This module defines the `WindowBackend` trait, which serves as the core
//! abstraction for creating and managing platform-specific windows.

use crate::core::event::event_handler::EventHandler;

pub trait WindowBackend<T, E: EventHandler<T>> {
    /// Runs the window's message loop.
    ///
    /// This method takes ownership of the window backend (`self: Box<Self>`)
    /// and is responsible for processing window messages until the window is
    /// closed.
    fn run(self: Box<Self>) -> anyhow::Result<()>;
}