//! # Event Handling
//!
//! This module defines the event handling system for the framework. It includes the
//! `EventHandler` trait, which provides a structured way to respond to window
//! messages, and the `RootEventHandler`, which composes multiple event handlers.

pub mod event_handler;
pub mod input_state;
pub mod modifier_key_handler;
pub mod key_id;
pub mod render_event_handler;
pub mod root_event_handler;

use crate::core::event::key_id::KeyId;
use crate::core::types::Size;

/// Represents a platform-agnostic GUI event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The window is requested to be closed.
    WindowClose,

    /// The window has been resized.
    WindowResize(Size),

    /// A keyboard key was pressed.
    KeyDown(KeyboardEvent),

    /// A keyboard key was released.
    KeyUp(KeyboardEvent),

    /// The mouse cursor has moved.
    MouseMove(MouseEvent),

    /// A mouse button was pressed.
    MouseDown(MouseEvent),

    /// A mouse button was released.
    MouseUp(MouseEvent),

    /// The window needs to be repainted.
    Paint,
}

/// Represents a mouse event.
#[derive(Debug, Clone, PartialEq)]
pub struct MouseEvent {
    /// The x-coordinate of the mouse cursor.
    pub x: i32,
    /// The y-coordinate of the mouse cursor.
    pub y: i32,
    /// The mouse button that was pressed or released.
    pub button: Option<MouseButton>,
}

/// Represents a mouse button.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// Represents a keyboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardEvent {
    /// The key that was pressed or released.
    pub key: KeyId,
}
