//! # Event Handling
//!
//! This module defines the core components of the event handling system. The system
//! is designed to be modular and extensible, allowing for a clear separation of
//! concerns when processing user input and window events.
//!
//! ## Core Components
//!
//! - **`Event`**: An enum that represents all possible events that can occur in the
//!   application, such as keyboard input, mouse movements, and window actions.
//!   It serves as a unified, platform-agnostic representation of events.
//!
//! - **`EventHandler`**: A trait that defines the interface for handling events.
//!   Developers can implement this trait to create custom logic for responding
//!   to specific events.
//!
//! - **`RootEventHandler`**: A concrete implementation of `EventHandler` that can
//!   compose multiple other event handlers. This allows for a modular design
//!   where different handlers can be responsible for different aspects of event
//!   processing (e.g., one for rendering, one for UI logic).
//!
//! - **`InputState`**: A struct that tracks the real-time state of modifier keys
//!   (Shift, Ctrl, Alt) and mouse buttons. This is useful for implementing
//!   complex interactions that depend on the current input state.
//!
//! ## Event Flow
//!
//! 1. The platform-specific window backend receives a native OS message.
//! 2. The message is translated into a platform-agnostic `Event`.
//! 3. The `Event` is passed to the `RootEventHandler`.
//! 4. The `RootEventHandler` dispatches the `Event` to all of its registered
//!    child handlers, allowing each one to process it.

pub mod event_handler;
pub mod input_state;
pub mod handlers;
pub mod key_id;

use crate::core::event::handlers::keyboard_handler::KeyboardEvent;
use crate::core::event::handlers::mouse_handler::MouseEvent;
use glam::UVec2;

/// Represents a platform-agnostic GUI event.
///
/// This enum encapsulates all possible events that an application can receive,
/// from window actions to user input. Each variant contains the necessary data
/// to handle the event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The user has requested to close the window (e.g., by clicking the 'X' button).
    ///
    /// The default behavior is to terminate the application's message loop.
    WindowClose,

    /// The window's client area has been resized.
    ///
    /// Contains the new size of the window in pixels.
    WindowResize(UVec2),

    /// A keyboard key was pressed.
    ///
    /// This event is triggered for raw key presses and provides detailed information
    /// about the key, including its virtual key code and the state of modifier keys.
    /// This is useful for implementing shortcuts or game controls.
    KeyDown(KeyboardEvent),

    /// A keyboard key was released.
    ///
    /// This event complements `KeyDown` and is triggered when a key is released.
    KeyUp(KeyboardEvent),

    /// A translated character was received from the keyboard.
    ///
    /// This event represents a character that has been processed by the OS's
    /// input method editor (IME). It is suitable for text input fields, as it
    /// correctly handles things like dead keys and complex script input.
    Character(char),

    /// The mouse cursor has moved over the window's client area.
    ///
    /// Contains detailed information about the mouse state, including its
    /// current position and which buttons are pressed.
    MouseMove(MouseEvent),

    /// A mouse button was pressed.
    ///
    /// Contains detailed information about the mouse state at the moment the
    /// button was pressed.
    MouseDown(MouseEvent),

    /// A mouse button was released.
    ///
    /// Contains detailed information about the mouse state at the moment the
    /// button was released.
    MouseUp(MouseEvent),

    /// The mouse wheel was scrolled.
    ///
    /// Contains the scroll delta. A positive value indicates scrolling forward
    /// (away from the user), and a negative value indicates scrolling backward
    /// (toward the user).
    MouseWheel(f32),

    /// The window's content needs to be repainted.
    ///
    /// This event is triggered whenever the OS determines that the window's
    /// client area is invalid and needs to be redrawn (e.g., after being
    /// uncovered or resized). The `RenderEventHandler` is responsible for
    /// handling this event.
    Paint,
}