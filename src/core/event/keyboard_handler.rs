use std::collections::HashSet;

use crate::core::backend::renderer::Renderer;
use crate::core::event::Event;
use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputState;
use crate::core::event::key_id::KeyId;

#[derive(Default)]
pub struct KeyboardInputHandler {
    pressed_keys: HashSet<KeyId>,
}

impl KeyboardInputHandler {
    pub fn is_key_pressed(&self, key: KeyId) -> bool {
        self.pressed_keys.contains(&key)
    }
}

impl<T: HasInputState> EventHandler<T> for KeyboardInputHandler {
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
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardEvent {
    /// The key that was pressed or released.
    pub key: KeyId,
}
