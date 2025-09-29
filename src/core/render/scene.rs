use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer; // Use the Renderer trait

/// A trait for types that contain a `Scene`.
///
/// This trait provides a standard way for the rendering system to access the
/// `Scene` from the application state.
pub trait HasScene {
    /// Returns a reference to the `Scene`.
    fn scene(&self) -> &Scene;
}

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
    /// This method is generic over any type that implements the `Drawable` trait.
    /// It automatically boxes the object and adds it to the scene's collection of
    /// trait objects.
    pub fn add_object<T: Drawable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    /// Draws all objects in the scene using the provided `Renderer`.
    ///
    /// This method iterates through all the `Drawable` objects in the scene and calls
    /// their respective `draw` methods, passing the renderer to each.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the `draw` calls fail.
    pub fn draw_all(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        for object in &self.objects {
            object.draw(renderer)?;
        }
        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}