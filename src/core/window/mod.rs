//! # Windowing
//!
//! This module provides the primary interface for creating and managing native windows.
//!
//! It abstracts away the platform-specific details of window creation and management,
//! offering a clean and unified API for developers. The main components are:
//!
//! - **`WindowBuilder`**: A fluent builder for configuring and constructing a `Window`.
//!   This is the recommended starting point for creating a new window. It allows you
//!   to set properties like title, size, and more before creating the window.
//!
//! - **`WindowConfig`**: A struct that holds all the configuration parameters for a
//!   window. It is used by the `WindowBuilder` to gather settings.
//!
//! The `platform` module contains the actual window implementation, which is selected
//! at compile time based on the target operating system.
//!
//! ## Example
//!
//! ```rust,no_run
//! use my_gui::core::window::{WindowBuilder, config::WindowConfig};
//! use my_gui::core::event::root_event_handler::RootEventHandler;
//!
//! // Define a simple application state
//! #[derive(Default)]
//! struct MyApp;
//!
//! fn main() -> anyhow::Result<()> {
//!     let app = MyApp;
//!     let event_handler = RootEventHandler::new();
//!     let config = WindowConfig {
//!         title: "My Awesome App".to_string(),
//!         width: 800,
//!         height: 600,
//!         ..Default::default()
//!     };
//!
//!     let window = WindowBuilder::from_config(config)
//!         .build(event_handler, app)?;
//!
//!     // Run the application's main loop.
//!     window.run()?;
//!
//!     Ok(())
//! }
//! ```

pub mod builder;
pub mod config;

pub use builder::WindowBuilder;