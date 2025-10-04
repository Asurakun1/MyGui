//! # Canvas: A Drawable Container
//!
//! This module defines the `Canvas` struct, a powerful `Drawable` container that
//! provides a local coordinate system and clipping for its child objects.

use crate::core::{backend::renderer::Renderer, render::drawable::Drawable};
use anyhow::Result;
use glam::Affine2;

/// A `Drawable` container that defines a local coordinate system and clipping region.
///
/// A `Canvas` is a fundamental building block for creating complex and encapsulated
/// UI components. It acts as a "sub-scene," allowing you to group, position, and
/// manage a collection of other `Drawable` objects as a single unit.
///
/// ## Key Features
///
/// - **Local Coordinate System**: When you add an object to a `Canvas`, its
///   position is relative to the top-left corner of the `Canvas` itself, not the
///   window. For example, drawing an object at `(0, 0)` places it at the top-left
///   of the `Canvas`, regardless of where the `Canvas` is located in the main scene.
///
/// - **Clipping**: The `Canvas` automatically clips its content. Any part of a
///   child object that extends beyond the `Canvas`'s boundaries will not be drawn.
///
/// - **Composition**: Since `Canvas` itself is `Drawable`, it can be nested inside
///   other `Canvas` objects, enabling complex, hierarchical UI structures.
///
/// ## Example: Creating a Simple Button
///
/// ```rust,no_run
/// use my_gui::core::render::objects::canvas::Canvas;
/// use my_gui::core::render::objects::primitives::Rectangle;
/// use my_gui::core::render::objects::text_object::TextObject;
/// use my_gui::core::render::scene::Scene;
/// use my_gui::core::render::color::Color;
///
/// // Create a canvas to represent a button at position (50, 50) in the window.
/// let mut button_canvas = Canvas::new(50.0, 50.0, 100.0, 30.0);
///
/// // Add a background rectangle at (0, 0) relative to the canvas.
/// let background = Rectangle::new(0.0, 0.0, 100.0, 30.0, Color::new(0.8, 0.8, 0.8, 1.0));
/// button_canvas.add_object(Box::new(background));
///
/// // Add text centered inside the canvas.
/// let label = TextObject::new("Click Me".to_string(), 25.0, 8.0, Color::BLACK);
/// button_canvas.add_object(Box::new(label));
///
/// // Add the fully composed button canvas to the main scene.
/// let mut scene = Scene::default();
/// scene.add_object(button_canvas);
/// ```
pub struct Canvas {
    objects: Vec<Box<dyn Drawable>>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Canvas {
    /// Creates a new `Canvas` with a specified position and size.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the canvas's top-left corner in its parent's coordinate system.
    /// * `y` - The y-coordinate of the canvas's top-left corner in its parent's coordinate system.
    /// * `width` - The width of the canvas's drawing and clipping area.
    /// * `height` - The height of the canvas's drawing and clipping area.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            objects: Vec::new(),
            x,
            y,
            width,
            height,
        }
    }

    /// Adds a `Drawable` object as a child of this canvas.
    ///
    /// The added object will be drawn relative to this canvas's top-left corner
    /// and will be clipped to its bounds.
    ///
    /// # Arguments
    ///
    /// * `object` - A `Box<dyn Drawable>` to be added to the canvas's list of children.
    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    /// Sets the position and size of the canvas.
    pub fn set_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    /// Sets the size of the canvas.
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Sets the position of the canvas.
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl Drawable for Canvas {
    /// Draws the canvas and all its contained objects.
    ///
    /// This method orchestrates the core functionality of the `Canvas`. It performs
    /// the following steps:
    /// 1.  Saves the current transformation matrix of the renderer.
    /// 2.  Applies a new translation to move the origin to the canvas's `(x, y)` position.
    /// 3.  Pushes a clipping rectangle that matches the canvas's bounds.
    /// 4.  Iterates through all child objects and calls their `draw` methods.
    /// 5.  Pops the clipping rectangle to remove the clip.
    /// 6.  Restores the original transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` to draw to.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the contained objects fail to draw.
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        // Save the current transformation state.
        let original_transform = renderer.get_transform();

        // Apply a translation to establish the local coordinate system for the canvas.
        let translation = Affine2::from_translation(glam::vec2(self.x, self.y));
        renderer.set_transform(&translation);

        // Apply a clip to constrain all subsequent drawing to the canvas bounds.
        renderer.push_axis_aligned_clip(0.0, 0.0, self.width, self.height);

        // Draw all child objects within the new transformed and clipped context.
        for object in &self.objects {
            object.draw(renderer)?;
        }

        // Restore the original rendering state by removing the clip and transform.
        renderer.pop_axis_aligned_clip();
        renderer.set_transform(&original_transform);

        Ok(())
    }
}