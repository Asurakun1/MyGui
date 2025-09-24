
use windows::{
    core::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::{D2D1_DRAW_TEXT_OPTIONS_NONE},
    Win32::Graphics::DirectWrite::{DWRITE_MEASURING_MODE_NATURAL},
};

use crate::render::drawable::Drawable;
use crate::render::drawing_context::DrawingContext;

/// A drawable object that represents a piece of text.
pub struct TextObject {
    /// The text to be rendered.
    pub text: String,
    /// The x-coordinate of the top-left corner of the text.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the text.
    pub y: f32,
}

impl TextObject {
    /// Creates a new `TextObject`.
    pub fn new(text: &str, x: f32, y: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
        }
    }
}

impl Drawable for TextObject {
    /// Draws the text to the given `DrawingContext`.
    fn draw(&self, context: &DrawingContext) -> Result<()> {
        // TODO: The layout rectangle is currently hardcoded to a large size.
        // For more complex scenarios, this should be calculated based on the
        // actual size of the text.
        let layout_rect = D2D_RECT_F { left: self.x, top: self.y, right: self.x + 1000.0, bottom: self.y + 1000.0 };
        let text_utf16: Vec<u16> = self.text.encode_utf16().collect();

        unsafe {
            context.render_target.DrawText(
                &text_utf16,
                context.text_format,
                &layout_rect,
                context.brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
        Ok(())
    }
}
