use crate::core::backend::renderer::Renderer;
use crate::core::event::{Event, event_handler::EventHandler};

#[derive(Debug, Default)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
}

pub trait HasMouseState {
    fn mouse_state(&self) -> &MouseState;
    fn mouse_state_mut(&mut self) -> &mut MouseState;
}

pub struct MouseInputHandler;

impl<T: HasMouseState> EventHandler<T> for MouseInputHandler {
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) {
        match event {
            Event::MouseMove(MouseEvent { x, y, .. }) => {
                let mouse_state = app.mouse_state_mut();
                mouse_state.x = *x;
                mouse_state.y = *y;
            }
            Event::MouseDown(MouseEvent { button, .. }) => {
                let mouse_state = app.mouse_state_mut();
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
                let mouse_state = app.mouse_state_mut();
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
