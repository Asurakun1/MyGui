//! # Built-in Event Handlers
//!
//! This module provides a collection of default [`EventHandler`] implementations
//! that manage common tasks like rendering, input state tracking, and event
//! composition. These handlers are designed to be modular and can be combined
//! within a [`RootEventHandler`] to build up the application's core logic.
//!
//! ## Available Handlers
//!
//! - **[`RootEventHandler`]**: The primary event handler that acts as a container
//!   for other handlers. It delegates incoming events to its children in sequence.
//!   This is the top-level handler passed to the `WindowBuilder`.
//!
//! - **[`RenderEventHandler`]**: A specialized handler that listens for the `Paint`
//!   event and orchestrates the rendering of the application's scene graph.
//!
//! - **[`KeyboardInputHandler`]**: A stateful handler that tracks which keyboard
//!   keys are currently pressed down. It also updates the global `InputState`
//!   for modifier keys (Shift, Ctrl, Alt).
//!
//! - **[`MouseInputHandler`]**: A stateful handler that tracks the mouse cursor's
//!   position and button states, updating the global `MouseState`.

pub mod keyboard_handler;
pub mod mouse_handler;
pub mod render_event_handler;
pub mod root_event_handler;