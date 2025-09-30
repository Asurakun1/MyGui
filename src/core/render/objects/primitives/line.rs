use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer; // Use the Renderer trait

/// A `Drawable` object that represents a line segment.
pub struct Line {
    pub p0_x: f32,
    pub p0_y: f32,
    pub p1_x: f32,
    pub p1_y: f32,
    pub stroke_width: f32,
}

impl Line {
    /// Creates a new `Line` with the specified start and end points, and stroke width.
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
    /// Draws the line to the render target using the provided `Renderer`.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_line(self)
    }
}