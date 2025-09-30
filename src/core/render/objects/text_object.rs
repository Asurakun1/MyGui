use anyhow::Result; // Explicitly use anyhow::Result

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer; // Use the Renderer trait

/// A `Drawable` object that represents a piece of text.
///
/// This struct holds the text string and its position, and it implements the `Drawable`
/// trait to render itself using Direct2D and DirectWrite.
pub struct TextObject {
    /// The text to be rendered.
    pub text: String,
    /// The x-coordinate of the top-left corner of the text layout box.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the text layout box.
    pub y: f32,
}

impl TextObject {
    /// Creates a new `TextObject` with the specified text and position.
    pub fn new(text: &str, x: f32, y: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
        }
    }
}

impl Drawable for TextObject {
    /// Draws the text to the render target using the provided `Renderer`.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to create the text layout.
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        renderer.draw_text(self)
    }
}