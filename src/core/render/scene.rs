use windows::core::Result;

use crate::core::render::drawable::Drawable;
use crate::core::render::drawing_context::DrawingContext;

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

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}
