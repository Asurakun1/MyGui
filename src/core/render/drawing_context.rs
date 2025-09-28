
use windows::{
    Win32::Graphics::Direct2D::ID2D1RenderTarget,
    Win32::Graphics::Direct2D::ID2D1SolidColorBrush,
    Win32::Graphics::DirectWrite::{IDWriteFactory, IDWriteTextFormat},
};

/// A context for drawing operations.
///
/// This struct bundles the necessary Direct2D resources for drawing, making it
/// convenient to pass them to `Drawable` objects.
pub struct DrawingContext<'a> {
    /// The render target to draw to.
    pub render_target: &'a ID2D1RenderTarget,
    /// The brush to use for drawing.
    pub brush: &'a ID2D1SolidColorBrush,
    /// The text format to use for drawing text.
    pub text_format: &'a IDWriteTextFormat,
    // The DirectWrite factory for creating text layouts.
    pub dwrite_factory: &'a IDWriteFactory,
}
