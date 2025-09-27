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
//! To create a new `my_gui` application, you need to create a `Window` and an `App` instance.
//! The `Window` is responsible for creating and managing the application window, while the `App`
//! contains the central application state.
//!
//! ```rust,no_run
//! use my_gui::{
//!     app::app::App,
//!     event::root_event_handler::RootEventHandler,
//!     window::{
//!         config::{WINDOW_TITLE, WINDOW_CLASS_NAME},
//!         window::Window,
//!     },
//! };
//! use windows::core::Result;
//!
//! fn main() -> Result<()> {
//!     let app = App::new();
//!     let event_handler = RootEventHandler::new();
//!     let window = Window::new(WINDOW_TITLE, WINDOW_CLASS_NAME, event_handler, app)?;
//!     let result = window.message_loop();
//!     std::mem::forget(window);
//!     result
//! }
//! ```

pub mod app;
pub mod event;
pub mod render;
pub mod window;
