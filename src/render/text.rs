use windows::Win32::Graphics::Gdi::HDC;

use super::component::Component;
use super::render_context::RenderContext;
use super::primitives::rect::Rect;
use super::primitives::line::Line;

pub struct TextComponent {
    pub rect: Rect,
    pub text: String,
}

impl TextComponent {
    pub fn new(lines: &[Line], text: &str) -> Option<Self> {
        let rect = Rect::from_lines(lines)?;
        Some(Self {
            rect,
            text: text.to_string(),
        })
    }
}

impl Component for TextComponent {
    fn draw(&self, render_context: &RenderContext, hdc: HDC) {
        render_context.draw_text(hdc, &self.text, self.rect.x, self.rect.y);
    }
}