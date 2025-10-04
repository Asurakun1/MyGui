//! # Rectangle Primitive
//!
//! This module defines the `Rectangle` struct, a `Drawable` primitive for rendering
//! a solid-color rectangle.

use crate::core::{
    backend::renderer::Renderer,
    render::{color::Color, drawable::Drawable},
};

/// A `Drawable` struct that represents a filled rectangle.
///
/// This struct defines a rectangle by the coordinates of its top-left corner
/// (`x`, `y`), its `width` and `height`, and its fill `color`. It serves as a
/// basic building block for many UI elements and graphical displays.
///
/// The `Rectangle` is a simple data container; it delegates the actual rendering
/// logic to the `draw_rectangle` method of a [`Renderer`].
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
}

impl Rectangle {
    /// Creates a new `Rectangle` with the specified position, size, and color.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the top-left corner.
    /// * `y` - The y-coordinate of the top-left corner.
    /// * `width` - The width of the rectangle.
    /// * `height` - The height of the rectangle.
    /// * `color` - The `Color` to fill the rectangle with.
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self { x, y, width, height, color }
    }
}

impl Drawable for Rectangle {
    /// Draws the rectangle by delegating to the active `Renderer`.
    ///
    /// This method calls the `draw_rectangle` method on the provided `Renderer`,
    /// passing a reference to itself.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_rectangle`
    /// method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_rectangle(self)
    }
}