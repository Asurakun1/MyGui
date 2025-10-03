//! # Keyboard Event Handling
//!
//! This module defines the structures and handlers related to keyboard input.
//!
//! ## Core Components
//!
//! - **`KeyboardEvent`**: A struct that encapsulates data about a keyboard event,
//!   specifically which key was involved. It is used in the `Event::KeyDown` and
//!   `Event::KeyUp` variants.
//!
//! - **`KeyboardInputHandler`**: An `EventHandler` that tracks the state of all
//!   pressed keys. It maintains a set of currently pressed keys and is also
//!   responsible for updating the main `InputState` for modifier keys (Shift,
//!   Ctrl, Alt). This handler is useful for implementing stateful keyboard logic,
//!   such as checking if a key is currently held down.

use std::collections::HashSet;

use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;
use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputState;
use crate::core::event::key_id::KeyId;

/// An `EventHandler` that tracks the state of pressed keys.
///
/// This handler listens for `KeyDown` and `KeyUp` events to maintain an internal
/// set of which keys are currently held down. It also updates the global
/// `InputState` for modifier keys (`Shift`, `Ctrl`, `Alt`).
///
/// This handler can be added to the `RootEventHandler` to enable key state tracking.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::event::handlers::keyboard_handler::KeyboardInputHandler;
/// use my_gui::core::event::key_id::KeyId;
///
/// let mut kbd_handler = KeyboardInputHandler::default();
/// // After processing some events...
/// if kbd_handler.is_key_pressed(KeyId::W) {
///     println!("Moving forward!");
/// }
/// ```
#[derive(Default)]
pub struct KeyboardInputHandler {
    pressed_keys: HashSet<KeyId>,
}

impl KeyboardInputHandler {
    /// Checks if a specific key is currently pressed.
    ///
    /// # Arguments
    ///
    /// * `key` - The `KeyId` to check.
    ///
    /// # Returns
    ///
    /// `true` if the key is in the set of pressed keys, `false` otherwise.
    pub fn is_key_pressed(&self, key: KeyId) -> bool {
        self.pressed_keys.contains(&key)
    }
}

impl<T: HasInputState> EventHandler<T> for KeyboardInputHandler {
    /// Updates the key state based on `KeyDown` and `KeyUp` events.
    ///
    /// When a `KeyDown` event occurs, the key is added to the `pressed_keys` set.
    /// If the key is a modifier, the corresponding flag in the application's
    /// `InputState` is set to `true`.
    ///
    /// When a `KeyUp` event occurs, the key is removed from the set, and the
    /// modifier flag in `InputState` is set to `false`.
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

/// Represents a keyboard event.
///
/// This struct is sent as part of the `Event::KeyDown` and `Event::KeyUp` variants.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardEvent {
    /// The platform-agnostic identifier of the key that was pressed or released.
    pub key: KeyId,
}