use windows::core::Result;

use crate::render::drawable::Drawable;
use crate::render::drawing_context::DrawingContext;

/// Represents a scene containing a collection of `Drawable` objects.
///
/// The `Scene` is the main container for everything that needs to be rendered
/// for a particular view. It holds a list of objects that implement the `Drawable`
/// trait, allowing for a heterogeneous collection of shapes, text, and other
/// graphical elements.
pub struct Scene {
    /// A vector of heap-allocated drawable objects.
    objects: Vec<Box<dyn Drawable>>,
}

impl Scene {
    /// Creates a new, empty `Scene`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds a `Drawable` object to the scene.
    ///
    /// The object is moved onto the heap and stored as a trait object (`Box<dyn Drawable>`),
    /// allowing the scene to manage objects of different concrete types.
    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    /// Draws all objects in the scene using the provided `DrawingContext`.
    ///
    /// This method iterates through all the `Drawable` objects in the scene and calls
    /// their respective `draw` methods, passing the drawing context to each.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the `draw` calls fail.
    pub fn draw_all(&self, context: &DrawingContext) -> Result<()> {
        for object in &self.objects {
            object.draw(context)?;
        }
        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
