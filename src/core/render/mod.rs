//! # Rendering Engine
//!
//! This module provides abstractions for rendering 2D graphics using Direct2D.
//! It defines a flexible system for drawing objects to the screen, organized around
//! a `Scene` that manages a collection of `Drawable` items.
//!
//! ## Key Components
//!
//! - **`Scene`**: A container for all objects that should be rendered in a window.
//! - **`Drawable`**: A trait for objects that can be drawn. Any object implementing
//!   this trait can be added to the `Scene`.
//! - **`DrawingContext`**: A struct that bundles together the necessary Direct2D resources
//!   (like the render target and brushes) for a drawing operation.
//! - **`Direct2DContext`**: Manages the lifetime of core Direct2D and DirectWrite
//!   factories and resources.
//! - **`objects`**: A submodule containing concrete implementations of the `Drawable`
//!   trait, such as `TextObject`.


pub mod drawing_context;
pub mod drawable;
pub mod objects;
pub mod scene;
