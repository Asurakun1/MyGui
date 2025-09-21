use windows::Win32::Graphics::Gdi::HDC;

use super::render_context::RenderContext;

pub trait Component {
    fn draw(&self, render_context: &RenderContext, hdc: HDC);
}