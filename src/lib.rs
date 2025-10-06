//! # my_gui: A Retained-Mode GUI Framework for Windows
//!
//! `my_gui` is a lightweight, modular framework for building simple GUI applications
//! in Rust. It leverages the raw Windows API for windowing and Direct2D for
//! hardware-accelerated rendering.
//!
//! ## Getting Started
//!
//! The easiest way to get started is to use the [`prelude`], which re-exports the
//! most common types and traits.
//!
//! Here is a basic example of a complete application:
//!
//! ```rust,no_run
//! use my_gui::prelude::*;
//!
//! // 1. Define the application's state. It must hold the input context and scene.
//! #[derive(Default)]
//! struct MyApp {
//!     input_context: InputContext,
//!     scene: Scene,
//! }
//!
//! // 2. Implement the required "has-a" traits to give the framework access
//! //    to the input context and scene.
//! impl HasInputContext for MyApp {
//!     fn input_context(&self) -> &InputContext { &self.input_context }
//!     fn input_context_mut(&mut self) -> &mut InputContext { &mut self.input_context }
//! }
//!
//! impl HasScene for MyApp {
//!     fn scene(&self) -> &Scene { &self.scene }
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     // 3. Create the application state.
//!     let mut app = MyApp::default();
//!
//!     // 4. Add some drawable objects to the scene.
//!     let rect = Rectangle::new(50.0, 50.0, 200.0, 100.0, Color::BLUE);
//!     let text = TextObject::new("Hello, World!".to_string(), 60.0, 85.0, Color::WHITE);
//!     app.scene.add_object(rect);
//!     app.scene.add_object(text);
//!
//!     // 5. Create the root event handler and add the default handlers.
//!     //    - `DefaultInputHandler`: Updates mouse and keyboard state.
//!     //    - `RenderEventHandler`: Handles `Paint` events and draws the scene.
//!     let mut event_handler = RootEventHandler::new();
//!     event_handler.add_handler(DefaultInputHandler::new());
//!     event_handler.add_handler(RenderEventHandler::new());
//!
//!     // 6. Use the WindowBuilder to configure and build the window.
//!     let window = WindowBuilder::new()
//!         .with_title("My GUI Application")
//!         .with_width(800)
//!         .with_height(600)
//!         .build(event_handler, app)?;
//!
//!     // 7. Run the application's main event loop.
//!     window.run()
//! }
//! ```
//!
//! ## Core Concepts
//!
//! The framework is built on a few key ideas:
//!
//! - **Retained-Mode Rendering**: You define a [`crate::prelude::Scene`] populated with [`crate::prelude::Drawable`]
//!   objects (like shapes and text). The framework "retains" this scene and is
//!   responsible for redrawing it whenever the window needs to be repainted. This
//!   simplifies rendering logic, as you only need to manage the state of your
//!   scene, not the individual draw calls each frame.
//!
//! - **Event-Driven Architecture**: Application logic is driven by events. You
//!   create [`crate::prelude::EventHandler`]s to respond to user input (like `MouseDown` or `KeyDown`)
//!   and window events (like `Paint` or `WindowClose`).
//!
//! - **Composition and Modularity**: The framework is designed to be composed.
//!   A [`crate::prelude::RootEventHandler`] can hold multiple specialized handlers, and `Drawable`
//!   objects can be grouped in a [`crate::prelude::Canvas`] to create complex, reusable components.
//!
//! ## Architecture
//!
//! The `core` module encapsulates all the framework's functionality:
//!
//! - [`core::window`]: Provides the [`crate::prelude::WindowBuilder`] for creating and configuring windows.
//!   This is the primary entry point for any application.
//! - [`core::event`]: Defines the event handling system, including the [`crate::prelude::Event`] enum, the
//!   [`crate::prelude::EventHandler`] trait, and a suite of built-in handlers for common tasks.
//! - [`core::render`]: Contains the retained-mode rendering abstractions: the [`crate::prelude::Scene`],
//!   the [`crate::prelude::Drawable`] trait, and a collection of built-in drawable objects.
//! - [`core::backend`]: Abstracts the rendering API via the [`crate::prelude::Renderer`] trait,
//!   decoupling the framework from a specific graphics API like Direct2D.
//! - [`core::platform`]: Isolates all platform-specific code, such as Win32 API calls
//!   for window creation and message processing.
//!
//! ## Prelude
//!
//! The [`prelude`] module provides a convenient way to import the most commonly
//! used types and traits with a single `use` statement:
//!
//! ```rust,no_run
//! use my_gui::prelude::*;
//! ```
//!
pub mod core;

/// A prelude for conveniently importing the most common types and traits.
pub mod prelude {
    pub use crate::core::prelude::*;
}
