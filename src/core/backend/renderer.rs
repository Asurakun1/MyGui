use crate::core::platform::RawWindowHandle;
use crate::core::render::objects::primitives::{
    ellipse::Ellipse, line::Line, rectangle::Rectangle,
};
use crate::core::render::objects::text_object::TextObject;
use glam::{Affine2, UVec2};

/// A trait for platform-agnostic rendering operations.
///
/// This trait defines the interface for drawing various shapes and text,
/// abstracting away the underlying graphics API.
pub trait Renderer {
    /// Creates resources that are tied to a specific rendering device.
    fn create_device_dependent_resources(&mut self, handle: RawWindowHandle) -> anyhow::Result<()>;

    /// Releases device-dependent resources.
    fn release_device_dependent_resources(&mut self);

    /// Returns the size of the render target, if available.
    fn get_render_target_size(&self) -> Option<UVec2>;

    /// Resizes the render target.
    fn resize_render_target(&mut self, new_size: UVec2) -> anyhow::Result<()>;

    /// Begins a new drawing operation.
    fn begin_draw(&mut self);

    /// Ends the current drawing operation.
    fn end_draw(&mut self) -> anyhow::Result<()>;

    /// Clears the render target with the specified color.
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    /// Pushes a clipping rectangle onto the render target.
    fn push_axis_aligned_clip(&mut self, x: f32, y: f32, width: f32, height: f32);

    /// Pops the last clipping rectangle from the render target.
    fn pop_axis_aligned_clip(&mut self);

    /// Sets the transformation matrix of the render target.
    fn set_transform(&mut self, matrix: &Affine2);

    /// Gets the transformation matrix of the render target.
    fn get_transform(&self) -> Affine2;

    /// Draws a rectangle.
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()>;

    /// Draws an ellipse.
    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()>;

    /// Draws a line.
    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()>;

    /// Draws text.
    fn draw_text(&mut self, text: &TextObject) -> anyhow::Result<()>;
}
