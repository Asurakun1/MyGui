//! # Event Handlers
//!
//! This module defines a composable event handling system for window messages.
//! The core is the `EventHandler` trait (defined in `crate::window_manager::event_handler`),
//! which provides a structured way to respond to different window events.
//!
//! ## Design
//!
//! The system is designed to be modular. The `RootEventHandler` is the main handler
//! passed to the `Window`. It, in turn, contains and delegates to more specialized
//! handlers, such as the `RenderEventHandler`. This allows for a clean separation of
//! concerns, where different parts of the application logic can be encapsulated in
//! their own handlers.

pub mod root_event_handler;
pub mod render_event_handler;
