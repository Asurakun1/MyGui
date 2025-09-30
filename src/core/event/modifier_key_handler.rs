#![allow(dead_code)]

use crate::core::{
    event::{
        Event,
        KeyboardEvent,
        event_handler::EventHandler,
        input_state::HasInputState,
        key_id::KeyId,
    },
    backend::renderer::Renderer,
};



pub struct ModifierKeyHandler;

impl<T: HasInputState> EventHandler<T> for ModifierKeyHandler {
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) {
        if let Event::KeyDown(KeyboardEvent { key }) = event {
            let input_state = app.input_state_mut();
            match key {
                KeyId::Shift => input_state.shift = true,
                KeyId::Control => input_state.ctrl = true,
                KeyId::Alt => input_state.alt = true,
                _ => {},
            }
        } else if let Event::KeyUp(KeyboardEvent { key }) = event {
            let input_state = app.input_state_mut();
            match key {
                KeyId::Shift => input_state.shift = false,
                KeyId::Control => input_state.ctrl = false,
                KeyId::Alt => input_state.alt = false,
                _ => {},
            }
        }
    }
}
