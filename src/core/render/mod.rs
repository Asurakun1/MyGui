//! # 2D Retained-Mode Rendering Engine
//!
//! This module provides the core abstractions for rendering 2D graphics using a
//! **retained-mode** approach. In this model, a persistent scene graph (a [`Scene`]
//! containing [`Drawable`] objects) is constructed and "retained" by the framework,
//! which then takes responsibility for rendering it efficiently.
//!
//! ## Core Concepts
//!
//! The rendering system is built on a few key abstractions:
//!
//! - **[`Drawable`]**: A fundamental trait representing any object that can be
//!   drawn on the screen. It has a single method, `draw`, which takes a [`Renderer`]
//!   and issues the necessary drawing commands.
//!
//! - **[`Scene`]**: A container that holds a heterogeneous collection of `Drawable`
//!   objects. This acts as the scene graph, managing all elements that need to
//!   be rendered.
//!
//! - **[`Renderer`]**: A trait (defined in `core::backend`) that provides a
//!   platform-agnostic interface for all drawing operations.
//!
//! - **[`Color`]**: A simple struct for representing RGBA colors in a
//!   platform-independent way.
//!
//! ## How It Works
//!
//! 1.  You create graphical objects (e.g., shapes, text, custom widgets) that
//!     implement the [`Drawable`] trait.
//! 2.  You add these `Drawable` objects to a [`Scene`] instance.
//! 3.  Your main application state struct must implement the `HasScene` trait to
//!     provide the framework with access to your scene.
//! 4.  When the window needs to be repainted (indicated by a `Paint` event), the
//!     `RenderEventHandler` retrieves the `Scene` and calls its `draw_all` method.
//! 5.  The `draw_all` method iterates through every `Drawable` object in the
//!     scene and calls its `draw` method, passing a `Renderer` to perform the
//!     actual drawing operations.
//!
//! ## Example
//!
//! ```rust,no_run
//! use my_gui::core::render::scene::{Scene, HasScene};
//! use my_gui::core::render::objects::primitives::Rectangle;
//! use my_gui::core::render::objects::text_object::TextObject;
//! use my_gui::core::render::color::Color;
//!
//! // 1. Define your application state with a Scene.
//! #[derive(Default)]
//! struct MyApp {
//!     scene: Scene,
//! }
//!
//! impl HasScene for MyApp {
//!     fn scene(&self) -> &Scene {
//!         &self.scene
//!     }
//! }
//!
//! // 2. Create a scene and add drawable objects to it.
//! let mut my_scene = Scene::new();
//! my_scene.add_object(Rectangle::new(10.0, 10.0, 100.0, 50.0, Color::RED));
//! my_scene.add_object(TextObject::new("Hello, Renderer!".to_string(), 20.0, 70.0, Color::BLACK));
//!
//! // 3. Store the scene in your app state.
//! let app = MyApp { scene: my_scene };
//! ```

pub mod color;
pub mod drawable;
pub mod objects;
pub mod scene;