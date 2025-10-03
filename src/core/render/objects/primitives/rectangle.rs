//! # Rectangle Primitive
//!
//! This module defines the `Rectangle` struct, a `Drawable` primitive for rendering
//! solid-color rectangles.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;
use crate::core::render::color::Color;

/// A `Drawable` struct that represents a rectangle.
///
/// This struct defines a rectangle by its top-left corner's position (`x`, `y`),
/// its dimensions (`width`, `height`), and its `color`.
/// It serves as a basic building block for many UI elements and graphical displays.
///
/// The `Rectangle` simply holds the geometric data and delegates the actual
/// rendering to the `Renderer`'s `draw_rectangle` method.
pub struct Rectangle {
    /// The x-coordinate of the top-left corner of the rectangle.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the rectangle.
    pub y: f32,
    /// The width of the rectangle.
    pub width: f32,
    /// The height of the rectangle.
    pub height: f32,
    /// The color of the rectangle, represented by a `Color` struct.
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
    /// * `color` - The `Color` of the rectangle.
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self { x, y, width, height, color }
    }
}

impl Drawable for Rectangle {
    /// Draws the rectangle by delegating to the `Renderer`.
    ///
    /// This method calls the `draw_rectangle` method on the provided `Renderer`,
    /// passing itself (which contains all the necessary geometric and color data).
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_rectangle` method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_rectangle(self)
    }
}