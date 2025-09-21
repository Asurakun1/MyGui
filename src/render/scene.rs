
use windows::core::Result;

use crate::render::drawable::Drawable;
use crate::render::drawing_context::DrawingContext;

/// Represents a scene containing a collection of drawable objects.
pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

impl Scene {
    /// Creates a new, empty `Scene`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds a drawable object to the scene.
    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    /// Draws all objects in the scene.
    pub fn draw_all(&self, context: &DrawingContext) -> Result<()> {
        for object in &self.objects {
            object.draw(context)?;
        }
        Ok(())
    }
}
