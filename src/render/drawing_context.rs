
use windows::{
    Win32::Graphics::Direct2D::ID2D1RenderTarget,
    Win32::Graphics::Direct2D::ID2D1SolidColorBrush,
    Win32::Graphics::DirectWrite::IDWriteTextFormat,
};

pub struct DrawingContext<'a> {
    pub render_target: &'a ID2D1RenderTarget,
    pub brush: &'a ID2D1SolidColorBrush,
    pub text_format: &'a IDWriteTextFormat,
}
