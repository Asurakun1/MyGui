//! # my_gui: A Simple Windows GUI Framework
//!
//! `my_gui` is a lightweight and modular framework for creating Windows GUI applications in Rust.
//! It provides a simple and intuitive API for creating windows, handling events, and rendering 2D graphics.
//!
//! ## Architecture
//!
//! The framework is divided into the following modules:
//!
//! - `core`: Encapsulates window creation, events, and rendering.
//!
//! ## Getting Started
//!
//! To create a new `my_gui` application, you use the `WindowBuilder` to configure and
//! create a `Window`. You also need to create an `App` instance and an event handler.
//!
//! ```rust,no_run
//! use my_gui::core::{
//!     backend::renderer::RendererConfig,
//!     event::{
//!         Event,
//!         event_handler::EventHandler,
//!         render_event_handler::RenderEventHandler,
//!         root_event_handler::RootEventHandler,
//!     },
//!     render::{
//!         objects::text_object::TextObject,
//!         scene::{HasScene, Scene},
//!     },
//!     window::{config::WindowConfig, WindowBuilder},
//! };
//! use anyhow::Result;
//!
//! // 1. Define the application state.
//! #[derive(Default)]
//! pub struct App {
//!     pub scene: Scene,
//! }
//!
//! impl HasScene for App {
//!     fn scene(&self) -> &Scene {
//!         &self.scene
//!     }
//! }
//!
//! impl App {
//!     pub fn new() -> Self {
//!         let mut scene = Scene::new();
//!         scene.add_object(TextObject::new("Hello, World!", 10.0, 10.0));
//!         Self { scene }
//!     }
//! }
//!
//! // 2. Define a custom event handler.
//! struct CustomEventHandler;
//!
//! impl<T> EventHandler<T> for CustomEventHandler {
//!     fn on_event(&mut self, _app: &mut T, event: &Event, _renderer: &mut my_gui::core::backend::renderer::Renderer) {
//!         println!("Received event: {:?}", event);
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let app = App::new();
//!
//!     let mut event_handler = RootEventHandler::new();
//!     event_handler.add_handler(Box::new(RenderEventHandler::new()));
//!     event_handler.add_handler(Box::new(CustomEventHandler));
//!
//!     let config = WindowConfig {
//!         title: "My Gui App".to_string(),
//!         ..Default::default()
//!     };
//!
//!     let window = WindowBuilder::from_config(config).build(event_handler, app)?;
//!
//!     let result = window.run();
//!
//!     std::mem::forget(window);
//!
//!     result
//! }
//! ```
//!
#[cfg(windows)]
pub mod core;
