//! # Line Primitive
//!
//! This module defines the `Line` struct, a `Drawable` primitive for rendering
//! a line segment with a specified thickness.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;
use crate::core::render::color::Color;

/// A `Drawable` struct that represents a line segment.
///
/// This struct defines a line by its start point (`p0_x`, `p0_y`),
/// its end point (`p1_x`, `p1_y`), its thickness (`stroke_width`), and its `color`.
///
/// Like other primitives, this struct is a simple data container that delegates
/// the rendering work to the `Renderer`'s `draw_line` method.
pub struct Line {
    /// The x-coordinate of the line's starting point.
    pub p0_x: f32,
    /// The y-coordinate of the line's starting point.
    pub p0_y: f32,
    /// The x-coordinate of the line's ending point.
    pub p1_x: f32,
    /// The y-coordinate of the line's ending point.
    pub p1_y: f32,
    /// The thickness (stroke width) of the line in pixels.
    pub stroke_width: f32,
    /// The color of the line, represented by a `Color` struct.
    pub color: Color,
}

impl Line {
    /// Creates a new `Line` with the specified start and end points, stroke width, and color.
    ///
    /// # Arguments
    ///
    /// * `p0_x` - The x-coordinate of the starting point.
    /// * `p0_y` - The y-coordinate of the starting point.
    /// * `p1_x` - The x-coordinate of the ending point.
    /// * `p1_y` - The y-coordinate of the ending point.
    /// * `stroke_width` - The thickness of the line in pixels.
    /// * `color` - The `Color` of the line.
    pub fn new(p0_x: f32, p0_y: f32, p1_x: f32, p1_y: f32, stroke_width: f32, color: Color) -> Self {
        Self { p0_x, p0_y, p1_x, p1_y, stroke_width, color }
    }


}

impl Drawable for Line {
    /// Draws the line by delegating to the `Renderer`.
    ///
    /// This method calls the `draw_line` method on the provided `Renderer`,
    /// passing itself (which contains all the necessary geometric and color data).
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_line` method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_line(self)
    }
}