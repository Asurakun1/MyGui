use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer; // Use the Renderer trait
use windows::core::Result;
use windows_numerics::Vector2;

/// A `Drawable` object that represents a line segment.
pub struct Line {
    pub p0: Vector2,
    pub p1: Vector2,
    pub stroke_width: f32,
}

impl Line {
    /// Creates a new `Line` with the specified start and end points, and stroke width.
    pub fn new(p0: Vector2, p1: Vector2, stroke_width: f32) -> Self {
        Self { p0, p1, stroke_width }
    }

    /// Creates a new `Line` with the specified start and end coordinates, and stroke width.
    pub fn new_with_xy(x0: f32, y0: f32, x1: f32, y1: f32, stroke_width: f32) -> Self {
        Self {
            p0: Vector2 { X: x0, Y: y0 },
            p1: Vector2 { X: x1, Y: y1 },
            stroke_width,
        }
    }
}

impl Drawable for Line {
    /// Draws the line to the render target using the provided `Renderer`.
    ///
    /// # Safety
    ///
    /// This function contains an `unsafe` block for calling the Direct2D `DrawLine`
    /// method. The caller must ensure that the `renderer` contains valid
    /// Direct2D resources.
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        renderer.draw_line(self)
    }
}