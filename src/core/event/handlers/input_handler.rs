//! # Input Handling
//!
//! This module provides the primary handlers and data structures for processing
//! raw keyboard and mouse input and tracking their state.

use crate::core::prelude::*;
use std::collections::HashSet;

// --- Keyboard ---

/// An [`EventHandler`] that tracks the real-time state of all pressed keys.
#[derive(Default)]
pub struct KeyboardInputHandler {
    pressed_keys: HashSet<KeyId>,
}

impl KeyboardInputHandler {
    /// Checks if a specific key is currently in the "pressed" state.
    pub fn is_key_pressed(&self, key: &KeyId) -> bool {
        self.pressed_keys.contains(key)
    }
}

impl<T: HasInputContext> EventHandler<T> for KeyboardInputHandler {
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) {
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                self.pressed_keys.insert(*key);
                let input_state = &mut app.input_context_mut().keyboard;
                match key {
                    KeyId::Shift => input_state.shift = true,
                    KeyId::Control => input_state.ctrl = true,
                    KeyId::Alt => input_state.alt = true,
                    _ => {}
                }
            }
            Event::KeyUp(KeyboardEvent { key }) => {
                self.pressed_keys.remove(key);
                let input_state = &mut app.input_context_mut().keyboard;
                match key {
                    KeyId::Shift => input_state.shift = false,
                    KeyId::Control => input_state.ctrl = false,
                    KeyId::Alt => input_state.alt = false,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

/// Represents a raw keyboard event (a key press or release).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardEvent {
    /// The platform-agnostic identifier of the key that was pressed or released.
    pub key: KeyId,
}

// --- Mouse ---

/// An [`EventHandler`] that updates the application's `InputContext`.
pub struct MouseInputHandler;

impl<T: HasInputContext> EventHandler<T> for MouseInputHandler {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    /// The x-coordinate of the mouse cursor at the time of the event.
    pub x: i32,
    /// The y-coordinate of the mouse cursor at the time of the event.
    pub y: i32,
    /// The specific mouse button associated with the event, if any.
    pub button: Option<MouseButton>,
}

/// Represents a physical button on a mouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}
