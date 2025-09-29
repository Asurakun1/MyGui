use crate::core::render::objects::primitives::{ellipse::Ellipse, line::Line, rectangle::Rectangle};
use crate::core::render::objects::text_object::TextObject;
use windows::Win32::Foundation::HWND;
use crate::core::types::Size; // Use the generic Size struct

/// Configuration for the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererConfig {
    /// Use the Direct2D renderer.
    Direct2D,
    // Add other renderer types here (e.g., OpenGL, Vulkan)
}

/// A trait for platform-agnostic rendering operations.
///
/// This trait defines the interface for drawing various shapes and text,
/// abstracting away the underlying graphics API.
pub trait Renderer {
    /// Creates resources that are tied to a specific rendering device (the `HWND`).
    fn create_device_dependent_resources(&mut self, hwnd: HWND) -> anyhow::Result<()>;

    /// Releases device-dependent resources.
    fn release_device_dependent_resources(&mut self);

    /// Returns the size of the render target, if available.
    fn get_render_target_size(&self) -> Option<Size>; // Changed to generic Size

    /// Resizes the render target.
    fn resize_render_target(&mut self, new_size: Size) -> anyhow::Result<()>; // Changed to generic Size

    /// Begins a new drawing operation.
    fn begin_draw(&mut self);

    /// Ends the current drawing operation.
    fn end_draw(&mut self) -> anyhow::Result<()>;

    /// Clears the render target with the specified color.
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    /// Draws a rectangle.
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()>;

    /// Draws an ellipse.
    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()>;

    /// Draws a line.
    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()>;

    /// Draws text.
    fn draw_text(&mut self, text: &TextObject) -> anyhow::Result<()>;
}
