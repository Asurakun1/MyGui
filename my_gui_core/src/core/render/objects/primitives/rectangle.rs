//! # Rectangle Primitive
//!
//! This module defines the `Rectangle` struct, a `Drawable` primitive for rendering
//! a solid-color rectangle.

use crate::core::prelude::*;

/// A `Drawable` struct that represents a filled rectangle.
///
/// This struct defines a rectangle by the coordinates of its top-left corner
/// (`x`, `y`), its `width` and `height`, and its fill `color`. It serves as a
/// basic building block for many UI elements and graphical displays.
///
/// The `Rectangle` is a simple data container; it delegates the actual rendering
/// logic to the `draw_rectangle` method of a [`Renderer`].
#[derive(Clone)]
pub struct Rectangle {
    /// The x-coordinate of the top-left corner of the rectangle.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the rectangle.
    pub y: f32,
    /// The width of the rectangle.
    pub width: f32,
    /// The height of the rectangle.
    pub height: f32,
    /// The fill color of the rectangle.
    pub color: Color,
    /// The border color of the rectangle (optional).
    pub border_color: Option<Color>,
    /// The border thickness of the rectangle (optional).
    pub border_thickness: Option<f32>,
}

impl Rectangle {
    /// Creates a new `Rectangle` with the specified position, size, color, and optional border properties.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the top-left corner.
    /// * `y` - The y-coordinate of the top-left corner.
    /// * `width` - The width of the rectangle.
    /// * `height` - The height of the rectangle.
    /// * `color` - The `Color` to fill the rectangle with.
    /// * `border_color` - The `Color` of the border (optional).
    /// * `border_thickness` - The thickness of the border (optional).
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        border_color: Option<Color>,
        border_thickness: Option<f32>,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            border_color,
            border_thickness,
        }
    }
}