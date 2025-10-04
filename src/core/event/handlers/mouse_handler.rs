//! # Mouse Event Handling
//!
//! This module provides the necessary structures and handlers for processing
//! all mouse-related input and tracking its state.
//!
//! ## Core Components
//!
//! - **[`MouseEvent`]**: A struct containing detailed information about a mouse
//!   event, such as cursor position and the button involved.
//!
//! - **[`MouseButton`]**: An enum representing the standard mouse buttons.
//!
//! - **[`MouseInputHandler`]**: A stateful [`EventHandler`] that listens for mouse
//!   events and updates the application's `InputContext` accordingly.

use crate::core::{
    backend::renderer::Renderer,
    event::{event_handler::EventHandler, input_state::HasInputContext, Event},
};

/// An [`EventHandler`] that updates the application's `InputContext`.
///
/// This handler listens for `MouseMove`, `MouseDown`, and `MouseUp` events and
/// updates the shared `InputContext` accordingly. It should be added to the
/// [`RootEventHandler`] to enable global mouse state tracking.
pub struct MouseInputHandler;

impl<T: HasInputContext> EventHandler<T> for MouseInputHandler {
    /// Updates the `InputContext` based on the received mouse event.
    /// - `MouseMove`: Updates the `x` and `y` coordinates.
    /// - `MouseDown`: Sets the corresponding button flag to `true`.
    /// - `MouseUp`: Sets the corresponding button flag to `false`.
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) {
        match event {
            Event::MouseMove(MouseEvent { x, y, .. }) => {
                let mouse_state = &mut app.input_context_mut().mouse;
                mouse_state.x = *x;
                mouse_state.y = *y;
            }
            Event::MouseDown(MouseEvent { button, .. }) => {
                let mouse_state = &mut app.input_context_mut().mouse;
                if let Some(button) = button {
                    match button {
                        MouseButton::Left => mouse_state.left_button = true,
                        MouseButton::Right => mouse_state.right_button = true,
                        MouseButton::Middle => mouse_state.middle_button = true,
                        _ => {}
                    }
                }
            }
            Event::MouseUp(MouseEvent { button, .. }) => {
                let mouse_state = &mut app.input_context_mut().mouse;
                if let Some(button) = button {
                    match button {
                        MouseButton::Left => mouse_state.left_button = false,
                        MouseButton::Right => mouse_state.right_button = false,
                        MouseButton::Middle => mouse_state.middle_button = false,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

/// Represents a specific mouse event.
///
/// This struct is sent as part of the [`Event::MouseMove`], [`Event::MouseDown`],
/// and [`Event::MouseUp`] variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    /// The x-coordinate of the mouse cursor at the time of the event.
    pub x: i32,
    /// The y-coordinate of the mouse cursor at the time of the event.
    pub y: i32,
    /// The specific mouse button associated with the event, if any.
    /// This is `None` for `MouseMove` events.
    pub button: Option<MouseButton>,
}

/// Represents a physical button on a mouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// The primary mouse button, usually the left one.
    Left,
    /// The secondary mouse button, usually the right one.
    Right,
    /// The middle mouse button, often part of the scroll wheel.
    Middle,
    /// A non-standard mouse button, identified by a platform-specific code.
    Other(u16),
}