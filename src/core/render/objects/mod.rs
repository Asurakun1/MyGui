//! # Built-in Drawable Objects
//!
//! This module provides a collection of pre-built structs that implement the
//! `Drawable` trait, making it easy to add common graphical elements to a `Scene`.
//!
//! ## Available Objects
//!
//! - **`primitives`**: A submodule containing basic geometric shapes.
//!   - `Rectangle`: A solid-color rectangle.
//!   - `Ellipse`: A solid-color ellipse.
//!   - `Line`: A line segment with a specified thickness.
//!
//! - **`TextObject`**: A simple object for rendering a single line of text at a
//!   specific position.
//!
//! - **`Canvas`**: A powerful container that acts as a drawable surface for other
//!   `Drawable` objects. A `Canvas` can be positioned and transformed, and it
//!   clips its children, effectively creating a local coordinate system. This is
//!   a key building block for creating complex UI components.
//!
//! ## Usage
//!
//! These objects can be instantiated, configured, and then added directly to a
//! `Scene` using the `scene.add_object()` method.
//!
//! ```rust,no_run
//! use my_gui::core::render::scene::Scene;
//! use my_gui::core::render::objects::primitives::{Rectangle, Line};
//! use my_gui::core::render::objects::text_object::TextObject;
//!
//! let mut scene = Scene::new();
//!
//! // Add a rectangle
//! let rect = Rectangle::new(10.0, 10.0, 200.0, 100.0);
//! scene.add_object(rect);
//!
//! // Add a line
//! let line = Line::new(10.0, 120.0, 210.0, 120.0, 2.0);
//! scene.add_object(line);
//!
//! // Add text
//! let text = TextObject::new("Hello, Objects!", 15.0, 15.0);
//! scene.add_object(text);
//! ```

pub mod primitives;
pub mod text_object;
pub mod canvas;