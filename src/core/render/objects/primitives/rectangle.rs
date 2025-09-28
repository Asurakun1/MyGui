use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use crate::core::render::drawable::Drawable;
use crate::core::render::drawing_context::DrawingContext;
use windows::core::Result;

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
    /// Draws the rectangle to the render target using the provided `DrawingContext`.
    ///
    /// # Safety
    ///
    /// This function contains an `unsafe` block for calling the Direct2D `FillRectangle`
    /// method. The caller must ensure that the `drawing_context` contains valid
    /// Direct2D resources.
    fn draw(&self, context: &DrawingContext) -> Result<()> {
        let rect = D2D_RECT_F {
            left: self.x,
            top: self.y,
            right: self.x + self.width,
            bottom: self.y + self.height,
        };

        unsafe {
            context.render_target.FillRectangle(&rect, context.brush);
        }

        Ok(())
    }
}