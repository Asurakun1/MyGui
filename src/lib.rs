//! # my_gui: A Simple Windows GUI Framework
//!
//! `my_gui` is a lightweight and modular framework for creating Windows GUI applications in Rust.
//! It provides a simple and intuitive API for creating windows, handling events, and rendering 2D graphics.
//!
//! ## Architecture
//!
//! The framework is divided into the following modules:
//!
//! - `app`: Contains the central application state.
//! - `core`: Encapsulates window creation, events, and rendering.
//!
//! ## Getting Started
//!
//! To create a new `my_gui` application, you use the `WindowBuilder` to configure and
//! create a `Window`. You also need to create an `App` instance and an event handler.
//!
//! ```rust,no_run
//! use my_gui::{
//!     app::App,
//!     core::event::root_event_handler::RootEventHandler,
//!     core::window::{WindowBuilder, config::WindowConfig},
//! };
//! use windows::core::Result;
//!
//! fn main() -> Result<()> {
//!     let app = App::new();
//!     let event_handler = RootEventHandler::new();
//!     let config = WindowConfig {
//!         title: "My Gui App".to_string(),
//!         ..Default::default()
//!     };
//!     let window = WindowBuilder::from_config(config)
//!         .build(event_handler, app)?;
//!     let result = window.run();
//!     std::mem::forget(window);
//!     result
//! }
//! ```

pub mod app;
#[cfg(windows)]
pub mod core;