use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer; // Use the Renderer trait

/// A `Drawable` object that represents an ellipse.
pub struct Ellipse {
    pub center_x: f32,
    pub center_y: f32,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl Ellipse {
    /// Creates a new `Ellipse` with the specified center and radii.
    pub fn new(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center_x,
            center_y,
            radius_x,
            radius_y,
        }
    }
}

impl Drawable for Ellipse {
    /// Draws the ellipse to the render target using the provided `Renderer`.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_ellipse(self)
    }
}