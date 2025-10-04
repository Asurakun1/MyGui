//! # Event Loop
//!
//! This module provides the `EventLoop`, which is responsible for running the
//! application's main event loop and dispatching messages.

use anyhow::Result;
use crate::core::platform::EventLoopBackend;
use crate::core::platform::win32::event_loop::Win32EventLoop;

/// The application's main event loop.
///
/// This struct is responsible for retrieving and dispatching messages from the
/// operating system.
pub struct EventLoop {
    backend: Box<dyn EventLoopBackend>,
}

impl EventLoop {
    /// Creates a new `EventLoop`.
    pub fn new() -> Self {
        #[cfg(target_os = "windows")]
        let backend = Box::new(Win32EventLoop::new());
        #[cfg(not(target_os = "windows"))]
        panic!("Unsupported platform");

        Self { backend }
    }

    /// Runs the main event loop.
    ///
    /// This function enters a loop that retrieves and dispatches messages from
    /// the window's message queue. The loop continues until `PostQuitMessage`
    /// is called (typically in response to a `WM_DESTROY` message), which
    /// causes `GetMessageW` to return `false`.
    pub fn run(&mut self) -> Result<()> {
        self.backend.run()
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}