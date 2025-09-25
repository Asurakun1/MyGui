
use windows::{
    core::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::{D2D1_DRAW_TEXT_OPTIONS_NONE},
    Win32::Graphics::DirectWrite::{DWRITE_MEASURING_MODE_NATURAL},
};

use crate::render::drawable::Drawable;
use crate::render::drawing_context::DrawingContext;

/// A `Drawable` object that represents a piece of text.
///
/// This struct holds the text string and its position, and it implements the `Drawable`
/// trait to render itself using Direct2D and DirectWrite.
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
    fn draw(&self, context: &DrawingContext) -> Result<()> {
        // Define the rectangle for the text layout.
        // TODO: The layout rectangle is currently hardcoded to a large size.
        // For more complex scenarios, this should be calculated based on the
        // actual size of the text using IDWriteTextLayout.
        let layout_rect = D2D_RECT_F { left: self.x, top: self.y, right: self.x + 1000.0, bottom: self.y + 1000.0 };
        let text_utf16: Vec<u16> = self.text.encode_utf16().collect();

        // The `DrawText` method is an `unsafe` FFI call to the Direct2D API.
        // Safety:
        // - `context.render_target` is a valid `ID2D1HwndRenderTarget`.
        // - `text_utf16` is a valid slice of UTF-16 code units.
        // - `context.text_format` is a valid `IDWriteTextFormat`.
        // - `layout_rect` is a valid `D2D_RECT_F`.
        // - `context.brush` is a valid `ID2D1SolidColorBrush`.
        // All pointers are valid and the function call is correct as per the WinAPI documentation.
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
