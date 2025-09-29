use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer; // Use the Renderer trait

/// A `Drawable` object that represents a rectangle.
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    /// Creates a new `Rectangle` with the specified position and size.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

impl Drawable for Rectangle {
    /// Draws the rectangle to the render target using the provided `Renderer`.
    ///
    /// # Safety
    ///
    /// This function contains an `unsafe` block for calling the Direct2D `FillRectangle`
    /// method. The caller must ensure that the `renderer` contains valid
    /// Direct2D resources.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_rectangle(self)
    }
}