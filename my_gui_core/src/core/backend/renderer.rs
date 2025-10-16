//! # The Renderer Trait
//!
//! This module defines the `Renderer` trait, which serves as the core abstraction
//! for all 2D drawing operations within the framework. By defining a generic
//! interface, it decouples the application's rendering logic from the specific
//! graphics API (e.g., Direct2D, OpenGL, Vulkan) used for implementation.

use std::any::Any;

use crate::core::prelude::*;
use crate::core::platform::RawWindowHandle;
use crate::core::render::objects::text_object::TextObject;
use glam::{Affine2, UVec2};

/// A platform-agnostic interface for 2D rendering operations.
///
/// This trait abstracts the underlying graphics API, providing a unified set of
/// commands for drawing shapes, text, and managing render state. [`crate::prelude::Drawable`]
/// objects use this trait to render themselves without needing to know the
/// specifics of the graphics backend.
///
/// The lifecycle of a `Renderer` is typically managed by the windowing backend,
/// which handles its creation, resizing, and cleanup. This ensures that rendering
/// resources are correctly initialized and released in sync with the native window.
pub trait Renderer {
    // --- Resource Management ---

    /// Creates and initializes resources that are tied to a specific rendering
    /// device, such as a GPU. This is typically called once when the renderer
    /// is first created or when the device has been lost and needs to be recreated.
    ///
    /// # Arguments
    /// * `handle` - A raw window handle for the window being rendered to. This is
    ///   necessary for the graphics API to connect to the native window.
    fn create_device_dependent_resources(&mut self, handle: RawWindowHandle) -> anyhow::Result<()>;

    /// Releases all device-dependent resources. This is called during cleanup
    /// or in response to a device loss event, ensuring that GPU memory and other
    /// handles are properly freed.
    fn release_device_dependent_resources(&mut self);

    // --- Render Target Management ---

    /// Returns the current size of the render target in pixels. This is useful
    /// for layout calculations that depend on the dimensions of the drawing surface.
    fn get_render_target_size(&self) -> Option<UVec2>;

    /// Resizes the render target, typically in response to a window resize event.
    /// This is a critical step to ensure the drawing surface matches the window's
    /// client area, preventing stretching or clipping.
    ///
    /// # Arguments
    /// * `new_size` - The new size of the render target in pixels.
    fn resize_render_target(&mut self, new_size: UVec2) -> anyhow::Result<()>;

    // --- Drawing Cycle ---

    /// Begins a drawing session. This must be called before any other drawing
    /// commands are issued. It prepares the render target for drawing, effectively
    /// locking it for the duration of the frame.
    fn begin_draw(&mut self);

    /// Ends the drawing session and presents the final rendered frame. This flushes
    /// all batched drawing commands to the GPU and, on platforms with a swap chain,
    /// swaps the back buffer to the front, making the new frame visible.
    ///
    /// # Errors
    /// Returns an error if the drawing session cannot be ended gracefully, such
    /// as in the case of a lost rendering device. Implementations should handle
    /// device loss by calling `release_device_dependent_resources`.
    fn end_draw(&mut self) -> anyhow::Result<()>;

    /// Clears the entire render target with the specified color. This is typically
    /// the first operation after `begin_draw` to ensure a clean slate for the new frame.
    fn clear(&mut self, color: &Color);

    // --- State Management (Transforms and Clipping) ---

    /// Pushes an axis-aligned clipping rectangle onto the clipping stack.
    ///
    /// Subsequent drawing operations will be clipped to the intersection of all
    /// clipping rectangles on the stack. This is essential for UI components
    /// like scroll viewers or canvases that need to constrain their content.
    fn push_axis_aligned_clip(&mut self, x: f32, y: f32, width: f32, height: f32);

    /// Pops the last clipping rectangle from the stack, restoring the previous one.
    /// This must be paired with a corresponding `push_axis_aligned_clip` call.
    fn pop_axis_aligned_clip(&mut self);

    /// Sets the current transformation matrix for the renderer.
    ///
    /// All subsequent drawing operations will be transformed by this matrix. This
    /// is fundamental for implementing translation, scaling, and rotation for
    /// objects like a [`crate::prelude::Canvas`] or custom UI elements.
    fn set_transform(&mut self, matrix: &Affine2);

    /// Gets the current transformation matrix. This is useful for constructing
    /// nested transformations or for hit-testing calculations.
    fn get_transform(&self) -> Affine2;

    // --- Primitive Drawing ---

    /// Draws a rectangle using the properties defined in the provided `Rectangle` struct.
    ///
    /// # Arguments
    ///
    /// * `rectangle` - A reference to the `Rectangle` struct containing the position, size, and color.
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()>;

    /// Draws the border of a rectangle.
    ///
    /// # Arguments
    ///
    /// * `rectangle` - A reference to the `Rectangle` struct containing the position, size, border color, and border thickness.
    fn draw_rectangle_border(&mut self, rectangle: &Rectangle) -> anyhow::Result<()>;

    /// Draws an ellipse using the properties defined in the provided `Ellipse` struct.
    ///
    /// # Arguments
    ///
    /// * `ellipse` - A reference to the `Ellipse` struct containing the center, radii, and color.
    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()>;

    /// Draws a line using the properties defined in the provided `Line` struct.
    ///
    /// # Arguments
    ///
    /// * `line` - A reference to the `Line` struct containing the start/end points, stroke width, and color.
    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()>;

    // --- Text Rendering ---

    /// Creates a backend-specific text layout object.
    ///
    /// This method takes a `TextObject` and generates a layout object (e.g.,
    /// `IDWriteTextLayout` in Direct2D) that can be cached and reused.
    ///
    /// # Arguments
    ///
    /// * `text` - The `TextObject` to create the layout for.
    ///
    /// # Returns
    ///
    /// A `Box<dyn Any>` containing the backend-specific layout object.
    fn create_text_layout(&self, text: &TextObject) -> anyhow::Result<Box<dyn Any>>;

    /// Draws a `TextObject` using its cached layout.
    ///
    /// This method assumes that the `layout` field of the `TextObject` has already
    /// been populated by a call to `create_text_layout`.
    ///
    /// # Arguments
    ///
    /// * `text` - The `TextObject` to draw.
    fn draw_text_layout(&mut self, text: &TextObject) -> anyhow::Result<()>;
}