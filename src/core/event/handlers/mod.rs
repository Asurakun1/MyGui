//! # Event Handlers
//!
//! This module provides a collection of built-in `EventHandler` implementations.
//! These handlers are designed to be composed within a `RootEventHandler` to
//! provide modular and extensible event processing.
//!
//! ## Available Handlers
//!
//! - **`RootEventHandler`**: The primary event handler that holds a collection of
//!   other handlers and dispatches events to them. This is the top-level handler
//!   that is passed to the `WindowBuilder`.
//!
//! - **`RenderEventHandler`**: A specialized handler responsible for processing the
//!   `Paint` event. It manages the rendering of the scene graph.
//!
//! - **`KeyboardEventHandler`**: (Not a public struct, but logic is in `wndproc`)
//!   Handles raw keyboard messages and translates them into `KeyDown`, `KeyUp`,
//!   and `Character` events.
//!
//! - **`MouseEventHandler`**: (Not a public struct, but logic is in `wndproc`)
//!   Handles raw mouse messages and translates them into `MouseMove`, `MouseDown`,

pub mod render_event_handler;
pub mod keyboard_handler;
pub mod mouse_handler;
pub mod root_event_handler;