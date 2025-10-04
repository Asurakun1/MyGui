//! # Keyboard Event Handling
//!
//! This module provides the primary handler and data structures for processing
//! raw keyboard input and tracking its state.
//!
//! ## Core Components
//!
//! - **[`KeyboardEvent`]**: A struct that encapsulates data about a key press
//!   or release, primarily the [`KeyId`] of the key involved. It is used in the
//!   `Event::KeyDown` and `Event::KeyUp` variants.
//!
//! - **[`KeyboardInputHandler`]**: A stateful [`EventHandler`] that maintains a
//!   set of all keys currently held down. It is also responsible for updating
//!   the global [`InputState`] for modifier keys (Shift, Ctrl, Alt). This
//!   handler is essential for any logic that needs to query if a key is
//!   pressed, such as in games or real-time applications.

use crate::core::{
    backend::renderer::Renderer,
    event::{event_handler::EventHandler, input_state::HasInputState, key_id::KeyId, Event},
};
use std::collections::HashSet;

/// An [`EventHandler`] that tracks the real-time state of all pressed keys.
///
/// This handler listens for `KeyDown` and `KeyUp` events to maintain an internal
/// `HashSet` of which keys are currently held down. It also updates the shared
/// [`InputState`] for modifier keys (`Shift`, `Ctrl`, `Alt`).
///
/// This handler should be added to the [`RootEventHandler`] to enable global key
/// state tracking.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::event::handlers::keyboard_handler::KeyboardInputHandler;
/// use my_gui::core::event::key_id::KeyId;
///
/// let mut kbd_handler = KeyboardInputHandler::default();
/// // After the handler processes a KeyDown event for 'W'...
/// if kbd_handler.is_key_pressed(&KeyId::W) {
///     println!("'W' key is currently held down!");
/// }
/// ```
#[derive(Default)]
pub struct KeyboardInputHandler {
    pressed_keys: HashSet<KeyId>,
}

impl KeyboardInputHandler {
    /// Checks if a specific key is currently in the "pressed" state.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the `KeyId` to check.
    ///
    /// # Returns
    ///
    /// `true` if the key is in the internal set of pressed keys, `false` otherwise.
    pub fn is_key_pressed(&self, key: &KeyId) -> bool {
        self.pressed_keys.contains(key)
    }
}

impl<T: HasInputState> EventHandler<T> for KeyboardInputHandler {
    /// Updates the key state based on `KeyDown` and `KeyUp` events.
    ///
    /// - On `KeyDown`: The key is added to the `pressed_keys` set. If the key is
    ///   a modifier, the corresponding flag in the application's `InputState` is
    ///   set to `true`.
    /// - On `KeyUp`: The key is removed from the set, and the corresponding
    ///   modifier flag in `InputState` is set to `false`.
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) {
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                self.pressed_keys.insert(*key);
                let input_state = app.input_state_mut();
                match key {
                    KeyId::Shift => input_state.shift = true,
                    KeyId::Control => input_state.ctrl = true,
                    KeyId::Alt => input_state.alt = true,
                    _ => {}
                }
            }
            Event::KeyUp(KeyboardEvent { key }) => {
                self.pressed_keys.remove(key);
                let input_state = app.input_state_mut();
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
///
/// This struct is sent as part of the [`Event::KeyDown`] and [`Event::KeyUp`]
/// variants. It provides direct information about which physical key was acted upon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardEvent {
    /// The platform-agnostic identifier of the key that was pressed or released.
    pub key: KeyId,
}