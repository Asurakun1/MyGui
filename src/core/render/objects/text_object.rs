//! # Text Object
//!
//! This module defines the `TextObject`, a `Drawable` primitive for rendering
//! a single line of text.

use std::any::Any;

use crate::core::prelude::*;
use crate::core::render::drawable::Drawable;

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
    /// A cached, backend-specific text layout object.
    pub layout: Option<Box<dyn Any>>,
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
        Self { text, x, y, color, layout: None }
    }
}

impl Drawable for TextObject {
    /// Draws the text by delegating to the active `Renderer`.
    ///
    /// This method implements a caching strategy for the text layout. On the first
    /// call, it creates a backend-specific text layout object using
    /// [`Renderer::create_text_layout`] and stores it. On subsequent calls, it
    /// reuses the cached layout, significantly improving performance for static text.
    ///
    /// If the text content of this `TextObject` is changed, the `layout` field
    /// should be set to `None` to force a recreation of the layout.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the layout creation or drawing fails.
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        if self.layout.is_none() {
            let layout = renderer.create_text_layout(self)?;
            self.layout = Some(layout);
        }
        renderer.draw_text_layout(self)
    }
}
