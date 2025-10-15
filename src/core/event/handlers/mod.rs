//! # Built-in Event Handlers
//!
//! This module provides a collection of default [`crate::prelude::EventHandler`] implementations
//! that manage common tasks like rendering, input state tracking, and event
//! composition. These handlers are designed to be modular and can be combined
//! within a [`crate::prelude::RootEventHandler`] to build up the application's core logic.
//!
//! ## Available Handlers
//!
//! - **[`crate::prelude::RootEventHandler`]**: The primary event handler that acts as a container
//!   for other handlers. It delegates incoming events to its children in sequence.
//!   This is the top-level handler passed to the `WindowBuilder`.
//!
//! - **[`crate::core::event::handlers::render_event_handler::RenderEventHandler`]**: A specialized handler that listens for the `Paint`
//!   event and orchestrates the rendering of the application's scene graph.
//!
//! - **[`crate::prelude::DefaultInputHandler`]**: A composite handler that tracks keyboard and mouse input.
//!   keys are currently pressed down, the mouse cursor's position, and button states.
//!   It also updates the global `InputState` and `MouseState`.

pub mod input_handler;
pub mod render_event_handler;
pub mod root_event_handler;
pub mod default_input_handler;
pub mod layout_event_handler;