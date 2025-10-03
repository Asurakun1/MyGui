//! # Primitive Shapes
//!
//! This module provides a collection of basic geometric shapes that implement the
//! `Drawable` trait. These primitives serve as the fundamental building blocks
//! for more complex graphics and UI components.
//!
//! ## Available Primitives
//!
//! - **`Rectangle`**: A rectangle defined by its position and size.
//! - **`Ellipse`**: An ellipse defined by its center and radii.
//! - **`Line`**: A line segment defined by two points and a stroke width.
//!
//! All primitives are rendered as solid-color shapes, with their color defined
//! as a property of the struct itself.

pub mod ellipse;
pub mod line;
pub mod rectangle;

pub use self::ellipse::Ellipse;
pub use self::line::Line;
pub use self::rectangle::Rectangle;