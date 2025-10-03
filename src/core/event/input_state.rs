//! # Input State Tracking
//!
//! This module provides a mechanism for tracking the real-time state of user
//! input devices, such as the keyboard's modifier keys.
//!
//! ## Core Components
//!
//! - **`InputState`**: A simple struct that holds boolean flags for the state
//!   of the `Shift`, `Ctrl`, and `Alt` keys. This struct is owned by the
//!   application's state and is updated by the window's message loop.
//!
//! - **`HasInputState`**: A trait that provides generic access to the `InputState`.
//!   Application state structs are required to implement this trait. This allows
//!   various parts of the framework, like event handlers, to query the input
//!   state without needing to know the concrete type of the application state.
//!
//! ## Usage
//!
//! The `InputState` is typically checked within an event handler to implement
//! input-sensitive logic, such as keyboard shortcuts (e.g., Ctrl+S) or modified
//! mouse actions (e.g., Shift+Click).
//!
//! ```rust,no_run
//! use my_gui::core::event::input_state::{InputState, HasInputState};
//!
//! // 1. Your application state struct.
//! #[derive(Default)]
//! struct MyApp {
//!     input: InputState,
//!     // ... other fields
//! }
//!
//! // 2. Implement `HasInputState` for your struct.
//! impl HasInputState for MyApp {
//!     fn input_state(&self) -> &InputState {
//!         &self.input
//!     }
//!
//!     fn input_state_mut(&mut self) -> &mut InputState {
//!         &mut self.input
//!     }
//! }
//!
//! // 3. Now you can check the input state in an event handler.
//! use my_gui::core::event::{Event, event_handler::EventHandler};
//!
//! struct MyEventHandler;
//!
//! impl<T: HasInputState> EventHandler<T> for MyEventHandler {
//!     fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn my_gui::core::backend::renderer::Renderer) {
//!         if let Event::KeyDown(key_event) = event {
//!             // Check for Ctrl+S
//!             if app.input_state().ctrl && key_event.key_id == my_gui::core::event::key_id::KeyId::S {
//!                 println!("Saving file...");
//!             }
//!         }
//!     }
//! }
//! ```

/// Holds the real-time state of keyboard modifier keys.
///
/// This struct is updated by the platform's window message loop in response to
/// key presses and releases for modifier keys.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InputState {
    /// `true` if the Shift key is currently pressed.
    pub shift: bool,
    /// `true` if the Control key is currently pressed.
    pub ctrl: bool,
    /// `true` if the Alt key (or Menu key on Windows) is currently pressed.
    pub alt: bool,
}

/// A trait for types that contain an `InputState`.
///
/// This trait must be implemented by the application's main state struct. It
/// provides a standardized way for the framework to access and update the
/// `InputState` without being coupled to the specific application state type.
pub trait HasInputState {
    /// Returns an immutable reference to the `InputState`.
    ///
    /// This allows components to query the current state of modifier keys.
    fn input_state(&self) -> &InputState;

    /// Returns a mutable reference to the `InputState`.
    ///
    /// This is used by the window's message loop to update the state of the
    /// modifier keys as they are pressed and released.
    fn input_state_mut(&mut self) -> &mut InputState;
}