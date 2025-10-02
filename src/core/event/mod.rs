//! # Event Handling
//!
//! This module defines the event handling system for the framework. It includes the
//! `EventHandler` trait, which provides a structured way to respond to window
//! messages, and the `RootEventHandler`, which composes multiple event handlers.

pub mod event_handler;
pub mod input_state;
pub mod handlers;
pub mod key_id;

use crate::core::event::handlers::keyboard_handler::KeyboardEvent;
use crate::core::event::handlers::mouse_handler::MouseEvent;
use glam::UVec2;


/// Represents a platform-agnostic GUI event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The window is requested to be closed.
    WindowClose,

    /// The window has been resized.
    WindowResize(UVec2),

    /// A keyboard key was pressed.
    KeyDown(KeyboardEvent),

    /// A keyboard key was released.
    KeyUp(KeyboardEvent),

    /// A translated character was received.
    Character(char),

    /// The mouse cursor has moved.
    MouseMove(MouseEvent),

    /// A mouse button was pressed.
    MouseDown(MouseEvent),

    /// A mouse button was released.
    MouseUp(MouseEvent),

    /// The mouse wheel was scrolled.
    MouseWheel(f32),

    /// The window needs to be repainted.
    Paint,
}
