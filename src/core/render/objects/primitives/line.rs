//! # Line Primitive
//!
//! This module defines the `Line` struct, a `Drawable` primitive for rendering
//! a line segment with a specified thickness.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;

/// A `Drawable` struct that represents a line segment.
///
/// This struct defines a line by its start point (`p0_x`, `p0_y`), its end point
/// (`p1_x`, `p1_y`), and its thickness (`stroke_width`).
///
/// Like other primitives, this struct is a simple data container that delegates
/// the rendering work to the `Renderer`'s `draw_line` method.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::render::objects::primitives::Line;
/// use my_gui::core::render::scene::Scene;
///
/// // Create a horizontal line from (10, 50) to (110, 50) with a thickness of 2 pixels.
/// let line = Line::new(10.0, 50.0, 110.0, 50.0, 2.0);
///
/// // Add it to a scene to be rendered.
/// let mut scene = Scene::new();
/// scene.add_object(line);
/// ```
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
}

impl Line {
    /// Creates a new `Line` with the specified start and end points, and stroke width.
    ///
    /// # Arguments
    ///
    /// * `p0_x` - The x-coordinate of the starting point.
    /// * `p0_y` - The y-coordinate of the starting point.
    /// * `p1_x` - The x-coordinate of the ending point.
    /// * `p1_y` - The y-coordinate of the ending point.
    /// * `stroke_width` - The thickness of the line.
    pub fn new(p0_x: f32, p0_y: f32, p1_x: f32, p1_y: f32, stroke_width: f32) -> Self {
        Self { p0_x, p0_y, p1_x, p1_y, stroke_width }
    }

    /// Creates a new `Line` with the specified start and end coordinates, and stroke width.
    pub fn new_with_xy(x0: f32, y0: f32, x1: f32, y1: f32, stroke_width: f32) -> Self {
        Self {
            p0_x: x0,
            p0_y: y0,
            p1_x: x1,
            p1_y: y1,
            stroke_width,
        }
    }
}

impl Drawable for Line {
    /// Draws the line by delegating to the `Renderer`.
    ///
    /// This method calls the `draw_line` method on the provided `Renderer`,
    /// passing the necessary geometric data.
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