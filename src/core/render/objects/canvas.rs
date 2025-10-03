//! # Canvas
//!
//! This module defines the `Canvas` struct, which acts as a container for drawable objects
//! with its own coordinate system and clipping boundaries.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;
use anyhow::Result;

/// A `Drawable` object that represents a rectangular drawing area.
///
/// A `Canvas` can contain other `Drawable` objects and provides its own
/// local coordinate system and clipping. This allows for creating reusable
/// UI components that manage their own rendering.
pub struct Canvas {
    objects: Vec<Box<dyn Drawable>>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Canvas {
    /// Creates a new `Canvas` with the specified position and size.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the top-left corner of the canvas.
    /// * `y` - The y-coordinate of the top-left corner of the canvas.
    /// * `width` - The width of the canvas.
    /// * `height` - The height of the canvas.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            objects: Vec::new(),
            x,
            y,
            width,
            height,
        }
    }

    /// Adds a `Drawable` object to the canvas.
    ///
    /// The added object will be drawn relative to the canvas's coordinate system.
    ///
    /// # Arguments
    ///
    /// * `object` - The `Drawable` object to add.
    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    /// Sets the position and size of the canvas.
    ///
    /// # Arguments
    ///
    /// * `x` - The new x-coordinate of the top-left corner of the canvas.
    /// * `y` - The new y-coordinate of the top-left corner of the canvas.
    /// * `width` - The new width of the canvas.
    /// * `height` - The new height of the canvas.
    pub fn set_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    /// Sets the size of the canvas.
    ///
    /// # Arguments
    ///
    /// * `width` - The new width of the canvas.
    /// * `height` - The new height of the canvas.
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Sets the position of the canvas.
    ///
    /// # Arguments
    ///
    /// * `x` - The new x-coordinate of the top-left corner of the canvas.
    /// * `y` - The new y-coordinate of the top-left corner of the canvas.
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

use glam::Affine2;

impl Drawable for Canvas {
    /// Draws the canvas and its contained objects to the renderer.
    ///
    /// This method applies a translation to the renderer's transform to position
    /// the canvas, sets up a clipping rectangle, draws all contained objects,
    /// and then restores the renderer's state.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` to draw to.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the contained objects fail to draw.
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        let original_transform = renderer.get_transform();

        let translation = Affine2::from_translation(glam::vec2(self.x, self.y));

        renderer.set_transform(&translation);

        renderer.push_axis_aligned_clip(0.0, 0.0, self.width, self.height);

        for object in &self.objects {
            object.draw(renderer)?;
        }

        renderer.pop_axis_aligned_clip();

        renderer.set_transform(&original_transform);

        Ok(())
    }
}
