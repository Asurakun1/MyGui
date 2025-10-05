//! # Windowing System
//!
//! This module provides the primary, high-level interface for creating and
//! managing native application windows.
//!
//! It abstracts away the platform-specific complexities of window creation,
//! offering a clean and unified API centered around the **builder pattern**.
//!
//! ## Core Components
//!
//! - **[`WindowBuilder`]**: A fluent builder for configuring and constructing a
//!   window. This is the main entry point for creating a new window. It allows
//!   you to set properties like title, size, and more before creation.
//!
//! - **[`WindowConfig`]**: A struct that holds all the configuration parameters
//!   for a window. While it can be used directly, it is most often managed
//!   internally by the `WindowBuilder`.
//!
//! The actual platform-specific window implementation is handled by the
//! `platform` module, which is selected at compile time.
//!
//! ## Example
//!
//! ```rust,no_run
//! use my_gui::prelude::*;
//!
//! // 1. Define the application's state.
//! #[derive(Default)]
//! struct MyApp {
//!     input_context: InputContext,
//!     scene: Scene,
//! }
//!
//! // 2. Implement the necessary state-access traits.
//! impl HasInputContext for MyApp {
//!     fn input_context(&self) -> &InputContext { &self.input_context }
//!     fn input_context_mut(&mut self) -> &mut InputContext { &mut self.input_context }
//! }
//!
//! impl HasScene for MyApp {
//!     fn scene(&self) -> &Scene { &self.scene }
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     // 3. Create the application state and the root event handler.
//!     let app = MyApp::default();
//!     let event_handler = RootEventHandler::new();
//!
//!     // 4. Use the WindowBuilder to configure and build the window.
//!     let window = WindowBuilder::new()
//!         .with_title("My Awesome App")
//!         .with_width(800)
//!         .with_height(600)
//!         .build(event_handler, app)?;
//!
//!     // 5. Run the application's main event loop.
//!     window.run()
//! }
//! ```

pub mod builder;
pub mod config;

pub use builder::WindowBuilder;

use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputContext;
use crate::core::platform::window_backend::WindowBackend;
use crate::core::window::config::WindowConfig;

/// Represents a native application window.
///
/// This struct is the high-level, public-facing representation of a window. It
/// encapsulates a platform-specific backend (`WindowBackend`) that handles the
/// actual OS-level interactions.
///
/// An instance of `Window` is typically created using a [`WindowBuilder`].
/// Once created, the `run` method is called to start the application's event loop.
///
/// # Type Parameters
///
/// - `T`: The application's state struct.
/// - `E`: The application's root event handler.
pub struct Window<T, E>
where
    T: 'static,
    E: 'static,
{
    /// The platform-specific window implementation.
    pub window_backend: Box<dyn WindowBackend<T, E>>,
}

impl<T: 'static + HasInputContext, E: 'static + EventHandler<T>> Window<T, E> {
    /// Creates a new window using the specified configuration, event handler, and app state.
    ///
    /// While this method can be used directly, it is often more convenient to
    /// use the [`WindowBuilder`] for a more fluent configuration experience.
    pub fn new(from_config: WindowConfig, event_handler: E, app: T) -> Result<Self, anyhow::Error> {
        let window_backend = WindowBuilder::from_config(from_config).build(event_handler, app)?;
        Ok(Self { window_backend })
    }

    /// Runs the window's main event loop.
    ///
    /// This method delegates to the underlying platform backend to start processing
    /// window messages. It blocks the current thread until the window is closed.
    pub fn run(self) -> anyhow::Result<()> {
        self.window_backend.run()
    }
}
