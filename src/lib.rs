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
//! - `event`: Handles all input and window events.
//! - `render`: Manages all rendering and drawing logic.
//! - `window`: Responsible for window creation and management.
//!
//! ## Getting Started
//!
//! To create a new `my_gui` application, you use the `WindowBuilder` to configure and
//! create a `Window`. You also need to create an `App` instance and an event handler.
//!
//! ```rust,no_run
//! use my_gui::{app::App, event::root_event_handler::RootEventHandler, window::WindowBuilder};
//! use windows::core::Result;
//!
//! fn main() -> Result<()> {
//!     let app = App::new();
//!     let event_handler = RootEventHandler::new();
//!     let window = WindowBuilder::new()
//!         .with_title("My App")
//!         .build(event_handler, app)?;
//!     let result = window.message_loop();
//!     std::mem::forget(window);
//!     result
//! }
//! ```

pub mod app;
pub mod event;
pub mod render;
pub mod window;