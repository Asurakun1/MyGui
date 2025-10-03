//! # Rendering Engine
//!
//! This module provides abstractions for rendering 2D graphics.
//! It defines a flexible system for drawing objects to the screen, organized around
//! a `Scene` that manages a collection of `Drawable` items.
//!
//! ## Key Components
//!
//! - **`Scene`**: A container for all objects that should be rendered in a window.
//! - **`Drawable`**: A trait for objects that can be drawn. Any object implementing
//!   this trait can be added to the `Scene`.
//! - **`objects`**: A submodule containing concrete implementations of the `Drawable`
//!   trait, such as `Rectangle`, `Ellipse`, `Line`, `TextObject`, and `Canvas`.

pub mod drawable;
pub mod objects;
pub mod scene;
