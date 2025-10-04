//! # Input State Tracking
//!
//! This module provides a centralized mechanism for tracking the real-time state
//! of user input devices, including both keyboard modifier keys and mouse state.
//!
//! ## Core Components
//!
//! - **[`InputState`]**: A simple struct holding boolean flags for the state of
//!   the `Shift`, `Ctrl`, and `Alt` keys.
//!
//! - **[`MouseState`]**: A struct that tracks the real-time state of the mouse,
//!   including its current coordinates and which buttons are pressed down.
//!
//! - **[`InputContext`]**: A composite struct that holds both `InputState` and
//!   `MouseState`, providing a single point of access for all input-related data.
//!   This is typically owned by the main application state struct.
//!
//! - **[`HasInputContext`]**: A trait that provides generic, decoupled access to the
//!   `InputContext`. Application state structs must implement this trait, allowing
//!   any part of the framework (like event handlers) to query and update input
//!   state without being tied to a concrete application state type.
//!
//! ## Usage
//!
//! The `InputContext` is most often queried within an event handler to implement
//! more complex, state-dependent logic, such as keyboard shortcuts (e.g., Ctrl+S)
//! or modified mouse actions (e.g., Shift+Click).
//!
//! ```rust,no_run
//! use my_gui::core::event::input_state::{InputContext, HasInputContext};
//!
//! // 1. Define your application state struct.
//! #[derive(Default)]
//! struct MyApp {
//!     input_context: InputContext,
//!     // ... other application fields
//! }
//!
//! // 2. Implement `HasInputContext` for your struct.
//! impl HasInputContext for MyApp {
//!     fn input_context(&self) -> &InputContext { &self.input_context }
//!     fn input_context_mut(&mut self) -> &mut InputContext { &mut self.input_context }
//! }
//!
//! // 3. Query the input state from within an event handler.
//! use my_gui::core::event::{Event, event_handler::EventHandler, key_id::KeyId};
//!
//! struct MyLogicHandler;
//!
//! impl<T: HasInputContext> EventHandler<T> for MyLogicHandler {
//!     fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn my_gui::core::backend::renderer::Renderer) {
//!         if let Event::KeyDown(key_event) = event {
//!             // Check for the "Ctrl+S" shortcut.
//!             if app.input_context().keyboard.ctrl && key_event.key_id == KeyId::S {
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

/// A composite struct that holds both `InputState` and `MouseState`.
///
/// This struct provides a single point of access for all input-related data,
/// simplifying the application state and event handler interfaces.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InputContext {
    /// The current state of keyboard modifier keys.
    pub keyboard: InputState,
    /// The current state of the mouse.
    pub mouse: MouseState,
}

/// A trait for types that contain an `InputContext`.
///
/// This "has-a" trait must be implemented by the application's main state struct.
/// It provides a standardized way for the framework's event handlers to access
/// and update the `InputContext` without being coupled to the application's
/// specific state type.
pub trait HasInputContext {
    /// Returns an immutable reference to the `InputContext`.
    ///
    /// This allows various components to query the current state of input devices.
    fn input_context(&self) -> &InputContext;

    /// Returns a mutable reference to the `InputContext`.
    ///
    /// This is used by input handlers to update the state of input devices.
    fn input_context_mut(&mut self) -> &mut InputContext;
}