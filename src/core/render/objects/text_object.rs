//! # Text Object
//!
//! This module defines the `TextObject`, a `Drawable` primitive for rendering
//! a single line of text.

use crate::core::{
    backend::renderer::Renderer,
    render::{color::Color, drawable::Drawable},
};
use anyhow::Result;

/// A `Drawable` struct for rendering a single line of text.
///
/// This struct acts as a simple container for a `String`, its top-left position
/// on the screen, and its `color`. It is a fundamental building block for
/// displaying text in an application.
///
/// The `TextObject` itself does not perform any complex layout calculations. It
/// simply holds the data and delegates the rendering work to the active
/// [`Renderer`] via its `draw` method. The renderer is then responsible for
/// handling font selection, text measurement, and rasterization.
pub struct TextObject {
    /// The text content to be rendered.
    pub text: String,
    /// The x-coordinate of the top-left corner of the text's layout box.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the text's layout box.
    pub y: f32,
    /// The color of the text.
    pub color: Color,
}

impl TextObject {
    /// Creates a new `TextObject` with the specified text, position, and color.
    ///
    /// # Arguments
    ///
    /// * `text` - The `String` to be rendered.
    /// * `x` - The x-coordinate where the text rendering will begin.
    /// * `y` - The y-coordinate where the text rendering will begin.
    /// * `color` - The `Color` of the text.
    pub fn new(text: String, x: f32, y: f32, color: Color) -> Self {
        Self { text, x, y, color }
    }
}

impl Drawable for TextObject {
    /// Draws the text by delegating to the active `Renderer`.
    ///
    /// This method calls the `draw_text` method on the provided `Renderer`,
    /// passing a reference to itself.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_text` method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        renderer.draw_text(self)
    }
}