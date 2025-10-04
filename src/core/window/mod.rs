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
//! use my_gui::core::window::WindowBuilder;
//! use my_gui::core::event::handlers::root_event_handler::RootEventHandler;
//! use my_gui::core::event::input_state::InputState;
//! use my_gui::core::event::handlers::mouse_handler::MouseState;
//!
//! // 1. Define the application's state.
//! #[derive(Default)]
//! struct MyApp {
//!     input_state: InputState,
//!     mouse_state: MouseState,
//! }
//!
//! // Implement the necessary state traits.
//! use my_gui::core::event::input_state::HasInputState;
//! use my_gui::core::event::handlers::mouse_handler::HasMouseState;
//!
//! impl HasInputState for MyApp {
//!     fn input_state(&self) -> &InputState { &self.input_state }
//!     fn input_state_mut(&mut self) -> &mut InputState { &mut self.input_state }
//! }
//!
//! impl HasMouseState for MyApp {
//!     fn mouse_state(&self) -> &MouseState { &self.mouse_state }
//!     fn mouse_state_mut(&mut self) -> &mut MouseState { &mut self.mouse_state }
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     // 2. Create the application state and the root event handler.
//!     let app = MyApp::default();
//!     let event_handler = RootEventHandler::new();
//!
//!     // 3. Use the WindowBuilder to configure and build the window.
//!     let window = WindowBuilder::new()
//!         .with_title("My Awesome App")
//!         .with_width(800)
//!         .with_height(600)
//!         .build(event_handler, app)?;
//!
//!     // 4. Run the application's main event loop.
//!     window.run()
//! }
//! ```

pub mod builder;
pub mod config;

pub use builder::WindowBuilder;

use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputState;
use crate::core::platform::window_backend::WindowBackend;
use crate::core::window::config::WindowConfig;

pub struct Window<T, E>
where
    T: 'static,
    E: 'static,
{
    pub window_backend: Box<dyn WindowBackend<T, E>>,
}

use crate::core::event::event_loop::EventLoop;

impl<T: 'static + HasInputState, E: 'static + EventHandler<T>> Window<T, E> {
    pub fn new(from_config: WindowConfig, event_handler: E, app: T) -> Result<Self, anyhow::Error> {
        let window_backend: Box<dyn WindowBackend<T, E>> =
            WindowBuilder::from_config(from_config).build(event_handler, app)?;
        Ok(Self { window_backend })
    }

    pub fn run(self) -> anyhow::Result<()> {
        let mut event_loop = EventLoop::new();
        event_loop.run()?;
        std::mem::forget(self);
        Ok(())
    }
}
