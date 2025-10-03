//! # my_gui: A Retained-Mode GUI Framework for Windows
//!
//! `my_gui` is a lightweight, modular framework for building simple GUI applications
//! in Rust. It leverages the raw Windows API for windowing and Direct2D for
//! hardware-accelerated rendering.
//!
//! ## Core Concepts
//!
//! The framework is built on a few key ideas:
//!
//! - **Retained-Mode Rendering**: You define a [`Scene`] populated with [`Drawable`]
//!   objects (like shapes and text). The framework "retains" this scene and is
//!   responsible for redrawing it whenever the window needs to be repainted.
//!
//! - **Event-Driven Architecture**: Application logic is driven by events. You
//!   create [`EventHandler`]s to respond to user input (like `MouseDown` or `KeyDown`)
//!   and window events (like `Paint` or `WindowClose`).
//!
//! - **Composition and Modularity**: The framework is designed to be composed.
//!   A [`RootEventHandler`] can hold multiple specialized handlers, and `Drawable`
//!   objects can be grouped in a [`Canvas`] to create complex, reusable components.
//!
//! ## Architecture
//!
//! The `core` module encapsulates all the framework's functionality:
//!
//! - [`core::window`]: Provides the [`WindowBuilder`] for creating and configuring windows.
//! - [`core::event`]: Defines the event handling system, including the [`Event`] enum and the
//!   [`EventHandler`] trait.
//! - [`core::render`]: Contains the [`Scene`], the [`Drawable`] trait, and a collection of
//!   built-in drawable objects.
//! - [`core::backend`]: Abstracts the rendering API via the [`Renderer`] trait.
//! - [`core::platform`]: Isolates platform-specific code, like Win32 API calls.
//!
//! ## Getting Started
//!
//! Here is a complete example of a simple application that displays some shapes and text.
//!
//! ```rust,no_run
//! use anyhow::Result;
//! use my_gui::core::{
//!     event::{
//!         event_handler::EventHandler,
//!         handlers::{
//!             keyboard_handler::KeyboardInputHandler, mouse_handler::{HasMouseState, MouseInputHandler, MouseState},
//!             render_event_handler::RenderEventHandler, root_event_handler::RootEventHandler,
//!         },
//!         input_state::{HasInputState, InputState},
//!         Event,
//!     },
//!     render::{
//!         color::Color,
//!         objects::{
//!             primitives::{Ellipse, Rectangle},
//!             text_object::TextObject,
//!         },
//!         scene::{HasScene, Scene},
//!     },
//!     window::WindowBuilder,
//! };
//!
//! // 1. Define your application's state.
//! // It must hold the scene and any input state trackers, and implement the
//! // corresponding "Has" traits.
//! #[derive(Default)]
//! pub struct MyApp {
//!     scene: Scene,
//!     input_state: InputState,
//!     mouse_state: MouseState,
//! }
//!
//! impl HasScene for MyApp {
//!     fn scene(&self) -> &Scene { &self.scene }
//! }
//!
//! impl HasInputState for MyApp {
//!     fn input_state(&self) -> &InputState { &self.input_state }
//!     fn input_state_mut(&mut self) -> &mut InputState { &mut self.input_state }
//! }
//!
//! impl HasMouseState for MyApp {
//!     fn mouse_state(&self) -> &MouseState { &self.mouse_state }
//!     fn mouse_state_mut(&mut self) -> &mut MouseState { &mut self.mouse_state }
//! }
//!
//! // 2. (Optional) Create a custom event handler for your application's logic.
//! pub struct AppLogicHandler;
//!
//! impl<T: HasMouseState> EventHandler<T> for AppLogicHandler {
//!     fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn my_gui::core::backend::renderer::Renderer) {
//!         if let Event::MouseDown(_) = event {
//!             // A logger (like simple_logger) would be needed for this to print.
//!             log::info!("Mouse clicked at ({}, {})", app.mouse_state().x, app.mouse_state().y);
//!         }
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     // 3. Create the scene and add drawable objects.
//!     let mut scene = Scene::new();
//!     scene.add_object(Rectangle::new(50.0, 50.0, 100.0, 80.0, Color::RED));
//!     scene.add_object(Ellipse::new(250.0, 150.0, 80.0, 80.0, Color::BLUE));
//!     scene.add_object(TextObject::new("Hello, my_gui!".to_string(), 50.0, 250.0, Color::BLACK));
//!
//!     // 4. Create the application state.
//!     let app = MyApp {
//!         scene,
//!         ..Default::default()
//!     };
//!
//!     // 5. Set up the root event handler and add the built-in handlers.
//!     let mut root_handler = RootEventHandler::new();
//!     root_handler.add_handler(Box::new(RenderEventHandler::new())); // Handles drawing
//!     root_handler.add_handler(Box::new(KeyboardInputHandler::default())); // Tracks keyboard state
//!     root_handler.add_handler(Box::new(MouseInputHandler));      // Tracks mouse state
//!     root_handler.add_handler(Box::new(AppLogicHandler));         // Your custom logic
//!
//!     // 6. Build and run the window.
//!     let window = WindowBuilder::new()
//!         .with_title("My GUI Application")
//!         .with_width(800)
//!         .with_height(600)
//!         .build(root_handler, app)?;
//!
//!     window.run()
//! }
//! ```
//!
#[cfg(windows)]
pub mod core;