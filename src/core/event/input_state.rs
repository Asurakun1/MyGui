//! # Input State Tracking
//!
//! This module provides a centralized mechanism for tracking the real-time state
//! of user input devices, specifically the keyboard's modifier keys.
//!
//! ## Core Components
//!
//! - **[`InputState`]**: A simple struct holding boolean flags for the state of
//!   the `Shift`, `Ctrl`, and `Alt` keys. This is typically owned by the main
//!   application state struct.
//!
//! - **[`HasInputState`]**: A trait that provides generic, decoupled access to the
//!   `InputState`. Application state structs must implement this trait, allowing
//!   any part of the framework (like event handlers) to query modifier key
//!   state without being tied to a concrete application state type.
//!
//! ## Usage
//!
//! The `InputState` is most often queried within an event handler to implement
//! more complex, state-dependent logic, such as keyboard shortcuts (e.g., Ctrl+S)
//! or modified mouse actions (e.g., Shift+Click).
//!
//! ```rust,no_run
//! use my_gui::core::event::input_state::{InputState, HasInputState};
//!
//! // 1. Define your application state struct.
//! #[derive(Default)]
//! struct MyApp {
//!     input: InputState,
//!     // ... other application fields
//! }
//!
//! // 2. Implement `HasInputState` for your struct.
//! impl HasInputState for MyApp {
//!     fn input_state(&self) -> &InputState { &self.input }
//!     fn input_state_mut(&mut self) -> &mut InputState { &mut self.input }
//! }
//!
//! // 3. Query the input state from within an event handler.
//! use my_gui::core::event::{Event, event_handler::EventHandler, key_id::KeyId};
//!
//! struct MyLogicHandler;
//!
//! impl<T: HasInputState> EventHandler<T> for MyLogicHandler {
//!     fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn my_gui::core::backend::renderer::Renderer) {
//!         if let Event::KeyDown(key_event) = event {
//!             // Check for the "Ctrl+S" shortcut.
//!             if app.input_state().ctrl && key_event.key_id == KeyId::S {
//!                 println!("Shortcut detected: Saving file...");
//!             }
//!         }
//!     }
//! }
//! ```

/// Holds the real-time state of keyboard modifier keys.
///
/// This struct is updated by the `KeyboardInputHandler` in response to `KeyDown`
/// and `KeyUp` events for modifier keys. It provides a snapshot of which
/// modifiers are currently held down.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InputState {
    /// `true` if the Shift key is currently pressed down.
    pub shift: bool,
    /// `true` if the Control key is currently pressed down.
    pub ctrl: bool,
    /// `true` if the Alt key (or Menu key on Windows) is currently pressed down.
    pub alt: bool,
}

/// A trait for types that contain an `InputState`.
///
/// This "has-a" trait must be implemented by the application's main state struct.
/// It provides a standardized way for the framework's event handlers to access
/// and update the `InputState` without being coupled to the application's
/// specific state type.
pub trait HasInputState {
    /// Returns an immutable reference to the `InputState`.
    ///
    /// This allows various components to query the current state of modifier keys.
    fn input_state(&self) -> &InputState;

    /// Returns a mutable reference to the `InputState`.
    ///
    /// This is used by the `KeyboardInputHandler` to update the state of the
    /// modifier keys as they are pressed and released.
    fn input_state_mut(&mut self) -> &mut InputState;
}