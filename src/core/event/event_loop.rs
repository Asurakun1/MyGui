#!/usr/bin/env rust-script
//! # Event Loop
//!
//! This module provides the main `EventLoop` for the application.

use crate::core::platform::event_loop::PlatformEventLoop;
use crate::core::platform::win32::event_loop::Win32EventLoop;
use anyhow::Result;

/// Manages the application's main event loop in a cross-platform way.
///
/// This struct acts as a high-level abstraction over the platform-specific
/// event loop implementation (e.g., `Win32EventLoop` on Windows). It is
/// responsible for creating and running the loop that retrieves and dispatches
/// messages for the application window.
pub struct EventLoop {
    platform_event_loop: Box<dyn PlatformEventLoop>,
}

impl Default for EventLoop {
    /// Creates a new `EventLoop` with the default platform-specific backend.
    fn default() -> Self {
        Self::new()
    }
}

impl EventLoop {
    /// Creates a new `EventLoop`.
    ///
    /// This function selects the appropriate platform-specific event loop
    /// implementation at compile time.
    pub fn new() -> Self {
        Self {
            platform_event_loop: Box::new(Win32EventLoop::new()),
        }
    }

    /// Starts the application's event loop.
    ///
    /// This method blocks the current thread and begins processing window
    /// messages. It will continue to run until the application is closed.
    ///
    /// # Errors
    ///
    /// Returns an error if the platform-specific event loop fails to initialize
    /// or run.
    pub fn run(&mut self) -> Result<()> {
        self.platform_event_loop.run()
    }
}
