//! # Primitive Shapes
//!
//! This module provides a collection of basic geometric shapes that implement the
//! [`Drawable`] trait. These primitives serve as the fundamental building blocks
//! for creating more complex graphics and UI components.
//!
//! ## Available Primitives
//!
//! - **[`Rectangle`]**: A solid-color rectangle defined by its position and size.
//! - **[`Ellipse`]**: A solid-color ellipse defined by its center and radii.
//! - **[`Line`]**: A line segment defined by two points and a stroke width.
//!
//! All primitives are simple data containers that delegate their drawing logic
//! to the active [`Renderer`].

pub mod ellipse;
pub mod line;
pub mod rectangle;

pub use self::{ellipse::Ellipse, line::Line, rectangle::Rectangle};