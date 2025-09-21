
use windows::{
    core::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::{D2D1_DRAW_TEXT_OPTIONS_NONE},
    Win32::Graphics::DirectWrite::{DWRITE_MEASURING_MODE_NATURAL},
};

use crate::render::drawable::Drawable;
use crate::render::drawing_context::DrawingContext;

pub struct TextObject {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

impl TextObject {
    pub fn new(text: &str, x: f32, y: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
        }
    }
}

impl Drawable for TextObject {
    fn draw(&self, context: &DrawingContext) -> Result<()> {
        let layout_rect = D2D_RECT_F { left: self.x, top: self.y, right: self.x + 1000.0, bottom: self.y + 1000.0 }; // Large enough rect for now
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
