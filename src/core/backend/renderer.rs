//! # The Renderer Trait
//!
//! This module defines the `Renderer` trait, which is the core abstraction for
//! all 2D drawing operations in the framework.

use crate::core::platform::RawWindowHandle;
use crate::core::render::objects::primitives::{
    ellipse::Ellipse, line::Line, rectangle::Rectangle,
};
use crate::core::render::objects::text_object::TextObject;
use glam::{Affine2, UVec2};

/// A platform-agnostic interface for 2D rendering operations.
///
/// This trait abstracts the underlying graphics API (e.g., Direct2D, Skia, etc.),
/// providing a unified set of commands for drawing shapes, text, and managing
/// render state. `Drawable` objects use this trait to render themselves without
/// needing to know the specifics of the graphics backend.
///
/// The `Renderer`'s lifecycle is typically managed by the windowing backend.
pub trait Renderer {
    // --- Resource Management ---

    /// Creates resources that are tied to a specific rendering device, such as a GPU.
    /// This is typically called once when the renderer is initialized.
    ///
    /// # Arguments
    /// * `handle` - A raw window handle for the window being rendered to.
    fn create_device_dependent_resources(&mut self, handle: RawWindowHandle) -> anyhow::Result<()>;

    /// Releases all device-dependent resources. This is called during cleanup.
    fn release_device_dependent_resources(&mut self);

    // --- Render Target Management ---

    /// Returns the current size of the render target in pixels.
    fn get_render_target_size(&self) -> Option<UVec2>;

    /// Resizes the render target. This is typically called when the window is resized.
    ///
    /// # Arguments
    /// * `new_size` - The new size of the render target.
    fn resize_render_target(&mut self, new_size: UVec2) -> anyhow::Result<()>;

    // --- Drawing Cycle ---

    /// Begins a drawing session. This must be called before any other drawing commands.
    fn begin_draw(&mut self);

    /// Ends the drawing session and presents the frame.
    ///
    /// # Errors
    /// Returns an error if the drawing session cannot be ended gracefully (e.g.,
    /// if the underlying device has been lost).
    fn end_draw(&mut self) -> anyhow::Result<()>;

    /// Clears the entire render target with the specified RGBA color.
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    // --- State Management (Transforms and Clipping) ---

    /// Pushes a clipping rectangle onto the clipping stack.
    ///
    /// Drawing operations will be clipped to the intersection of all clipping
    /// rectangles on the stack. This is essential for UI components like scroll
    /// viewers or canvases.
    fn push_axis_aligned_clip(&mut self, x: f32, y: f32, width: f32, height: f32);

    /// Pops the last clipping rectangle from the stack, restoring the previous one.
    fn pop_axis_aligned_clip(&mut self);

    /// Sets the current transformation matrix for the renderer.
    ///
    /// All subsequent drawing operations will be transformed by this matrix. This
    /// is used to implement translation, scaling, and rotation for objects like a `Canvas`.
    fn set_transform(&mut self, matrix: &Affine2);

    /// Gets the current transformation matrix.
    fn get_transform(&self) -> Affine2;

    // --- Primitive Drawing ---

    /// Draws a rectangle.
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()>;

    /// Draws an ellipse.
    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()>;

    /// Draws a line.
    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()>;

    /// Draws a `TextObject`. The renderer is responsible for font and layout.
    fn draw_text(&mut self, text: &TextObject) -> anyhow::Result<()>;
}