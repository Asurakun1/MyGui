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
//! - **[`MouseState`]**: A struct that tracks the real-time state of the mouse,
//!   including its current coordinates and which buttons are pressed down.
//!
//! - **[`HasMouseState`]**: A trait that application state structs must implement
//!   to provide generic, decoupled access to the `MouseState`.
//!
//! - **[`MouseInputHandler`]**: A stateful [`EventHandler`] that listens for mouse
//!   events and updates the application's `MouseState` accordingly.

use crate::core::{
    backend::renderer::Renderer,
    event::{event_handler::EventHandler, Event},
};

/// Holds the real-time state of the mouse.
///
/// This struct is updated by the `MouseInputHandler` in response to mouse events.
/// It tracks the cursor's current position relative to the window's client area
/// and the state of the primary mouse buttons.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MouseState {
    /// The current x-coordinate of the mouse cursor.
    pub x: i32,
    /// The current y-coordinate of the mouse cursor.
    pub y: i32,
    /// `true` if the left mouse button is currently pressed down.
    pub left_button: bool,
    /// `true` if the right mouse button is currently pressed down.
    pub right_button: bool,
    /// `true` if the middle mouse button is currently pressed down.
    pub middle_button: bool,
}

/// A trait for types that contain a `MouseState`.
///
/// This "has-a" trait must be implemented by the application's main state struct
/// to allow the framework's handlers to access and update the mouse state in a
/// decoupled manner.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::event::handlers::mouse_handler::{MouseState, HasMouseState};
///
/// #[derive(Default)]
/// struct MyApp {
///     mouse: MouseState,
///     // ... other application fields
/// }
///
/// impl HasMouseState for MyApp {
///     fn mouse_state(&self) -> &MouseState { &self.mouse }
///     fn mouse_state_mut(&mut self) -> &mut MouseState { &mut self.mouse }
/// }
/// ```
pub trait HasMouseState {
    /// Returns an immutable reference to the `MouseState`.
    fn mouse_state(&self) -> &MouseState;
    /// Returns a mutable reference to the `MouseState`.
    fn mouse_state_mut(&mut self) -> &mut MouseState;
}

/// An [`EventHandler`] that updates the application's `MouseState`.
///
/// This handler listens for `MouseMove`, `MouseDown`, and `MouseUp` events and
/// updates the shared `MouseState` accordingly. It should be added to the
/// [`RootEventHandler`] to enable global mouse state tracking.
pub struct MouseInputHandler;

impl<T: HasMouseState> EventHandler<T> for MouseInputHandler {
    /// Updates the `MouseState` based on the received mouse event.
    /// - `MouseMove`: Updates the `x` and `y` coordinates.
    /// - `MouseDown`: Sets the corresponding button flag to `true`.
    /// - `MouseUp`: Sets the corresponding button flag to `false`.
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