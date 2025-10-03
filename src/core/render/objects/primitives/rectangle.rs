//! # Rectangle Primitive
//!
//! This module defines the `Rectangle` struct, a `Drawable` primitive for rendering
//! solid-color rectangles.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;

/// A `Drawable` struct that represents a rectangle.
///
/// This struct defines a rectangle by its top-left corner's position (`x`, `y`)
/// and its dimensions (`width`, `height`). It serves as a basic building block
/// for many UI elements and graphical displays.
///
/// The `Rectangle` simply holds the geometric data and delegates the actual
/// rendering to the `Renderer`'s `draw_rectangle` method.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::render::objects::primitives::Rectangle;
/// use my_gui::core::render::scene::Scene;
///
/// // Create a rectangle at (10, 10) with a size of 100x50 pixels.
/// let rect = Rectangle::new(10.0, 10.0, 100.0, 50.0);
///
/// // Add it to a scene to be rendered.
/// let mut scene = Scene::new();
/// scene.add_object(rect);
/// ```
pub struct Rectangle {
    /// The x-coordinate of the top-left corner of the rectangle.
    pub x: f32,
    /// The y-coordinate of the top-left corner of the rectangle.
    pub y: f32,
    /// The width of the rectangle.
    pub width: f32,
    /// The height of the rectangle.
    pub height: f32,
}

impl Rectangle {
    /// Creates a new `Rectangle` with the specified position and size.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

impl Drawable for Rectangle {
    /// Draws the rectangle by delegating to the `Renderer`.
    ///
    /// This method calls the `draw_rectangle` method on the provided `Renderer`,
    /// passing itself as the data source for the rectangle's properties.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_rectangle` method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_rectangle(self)
    }
}