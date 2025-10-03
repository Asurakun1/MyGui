//! # 2D Rendering Engine
//!
//! This module provides the core abstractions for rendering 2D graphics. It is
//! designed around a retained-mode graphics model, where a persistent scene graph
//! (`Scene`) is maintained and rendered by the framework.
//!
//! ## Core Concepts
//!
//! The rendering system is built on two primary abstractions:
//!
//! - **`Drawable`**: A trait representing any object that can be drawn on the screen.
//!   It has a single method, `draw`, which takes a `Renderer` and performs the
//!   necessary drawing operations.
//!
//! - **`Scene`**: A container that holds a collection of `Drawable` objects. The `Scene`
//!   is responsible for managing all the elements that need to be rendered. The
//!   `RenderEventHandler` uses the `Scene` to draw the entire frame.
//!
//! ## How It Works
//!
//! 1.  You create objects that you want to draw. These objects must implement the
//!     `Drawable` trait. The `objects` submodule provides several built-in shapes
//!     and text objects.
//! 2.  You add these `Drawable` objects to a `Scene`.
//! 3.  Your application state struct must implement the `HasScene` trait to give the
//!     rendering system access to your `Scene`.
//! 4.  When the window needs to be repainted, the `RenderEventHandler` gets the `Scene`
//!     from your application state and calls `draw_all`, which in turn calls the `draw`
//!     method on every object in the scene.
//!
//! ## Example
//!
//! ```rust,no_run
//! use my_gui::core::render::scene::{Scene, HasScene};
//! use my_gui::core::render::objects::rectangle::Rectangle;
//! use my_gui::core::render::objects::text_object::TextObject;
//!
//! // 1. Define your application state with a Scene.
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
//! my_scene.add_object(Rectangle::new(10.0, 10.0, 100.0, 50.0));
//! my_scene.add_object(TextObject::new("Hello, Renderer!", 20.0, 70.0));
//!
//! // 3. Store the scene in your app state.
//! let app = MyApp { scene: my_scene };
//! ```

pub mod drawable;
pub mod objects;
pub mod scene;