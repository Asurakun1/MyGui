//! # Text Object
//!
//! This module defines the `TextObject`, a simple `Drawable` for rendering text.

use anyhow::Result;
use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;

/// A `Drawable` struct for rendering a single line of text.
///
/// This struct acts as a simple container for a string and its position on the
/// screen. It is a fundamental building block for displaying text in an application.
///
/// The `TextObject` itself does not contain any complex layout logic. It simply
/// holds the data and delegates the rendering work to the active `Renderer` via
/// its `draw` method. The renderer is then responsible for handling font selection,
/// text layout, and drawing.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::render::objects::text_object::TextObject;
/// use my_gui::core::render::scene::Scene;
///
/// // Create a text object.
/// let greeting = TextObject::new("Hello, World!", 10.0, 20.0);
///
/// // Add it to a scene to be rendered.
/// let mut scene = Scene::new();
/// scene.add_object(greeting);
/// ```
pub struct TextObject {
    /// The text content to be rendered.
    pub text: String,
    /// The x-coordinate of the top-left corner of the text's layout rectangle.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the text's layout rectangle.
    pub y: f32,
}

impl TextObject {
    /// Creates a new `TextObject` with the specified text and position.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice to be rendered.
    /// * `x` - The x-coordinate where the text rendering will begin.
    /// * `y` - The y-coordinate where the text rendering will begin.
    pub fn new(text: &str, x: f32, y: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
        }
    }
}

impl Drawable for TextObject {
    /// Draws the text by delegating to the `Renderer`.
    ///
    /// This method simply calls the `draw_text` method on the provided `Renderer`,
    /// passing itself as the data source for the text, position, and other properties
    /// that the renderer might need.
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