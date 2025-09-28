use windows::{core::*, Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE};
use windows_numerics::Vector2;

use crate::core::render::drawable::Drawable;
use crate::core::render::drawing_context::DrawingContext;

/// A `Drawable` object that represents a piece of text.
///
/// This struct holds the text string and its position, and it implements the `Drawable`
/// trait to render itself using Direct2D and DirectWrite.
pub struct TextObject {
    /// The text to be rendered.
    pub text: String,
    /// The x-coordinate of the top-left corner of the text layout box.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the text layout box.
    pub y: f32,
}

impl TextObject {
    /// Creates a new `TextObject` with the specified text and position.
    pub fn new(text: &str, x: f32, y: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
        }
    }
}

impl Drawable for TextObject {
    /// Draws the text to the render target using the provided `DrawingContext`.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to create the text layout.
    ///
    /// # Safety
    ///
    /// This function contains `unsafe` blocks for creating the text layout and drawing
    /// the text. The caller must ensure that the `drawing_context` contains valid
    /// Direct2D and DirectWrite resources.
    fn draw(&self, context: &DrawingContext) -> Result<()> {
        let text_utf16: Vec<u16> = self.text.encode_utf16().collect();

        let size = unsafe { context.render_target.GetSize() };

        let text_layout = unsafe {
            context.dwrite_factory.CreateTextLayout(
                &text_utf16,
                context.text_format,
                size.width,
                size.height,
            )?
        };

        let origin = Vector2 { X: self.x, Y: self.y };

        unsafe {
            context.render_target.DrawTextLayout(
                origin,
                &text_layout,
                context.brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
            );
        }

        Ok(())
    }
}