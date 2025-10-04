//! # Scene Management
//!
//! This module provides the `Scene` struct, which acts as the main container
//! for all `Drawable` objects in the retained-mode rendering model. It also
//! defines the `HasScene` trait for generic access to the scene.

use crate::core::backend::renderer::Renderer;
use crate::core::render::drawable::Drawable;

/// A trait for application state types that contain a `Scene`.
///
/// This "has-a" trait creates a generic interface for the rendering system to
/// access the `Scene` without being coupled to the concrete type of the
/// application's state struct. It is a required trait bound for the application
/// state (`T`) used by the `RenderEventHandler`.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::render::scene::{Scene, HasScene};
///
/// // Your application's main state struct.
/// #[derive(Default)]
/// struct MyApp {
///     scene: Scene,
///     // ... other state fields
/// }
///
/// // Implement `HasScene` to provide access to the scene field.
/// impl HasScene for MyApp {
///     fn scene(&self) -> &Scene {
///         &self.scene
///     }
/// }
/// ```
pub trait HasScene {
    /// Returns an immutable reference to the `Scene`.
    fn scene(&self) -> &Scene;
}

/// A scene graph containing a collection of `Drawable` objects.
///
/// The `Scene` is the central container for all graphical elements that are
/// rendered in a window. It maintains a list of `Drawable` trait objects,
/// allowing it to hold a heterogeneous collection of different shapes, text,
/// images, and custom widgets.
///
/// In the retained-mode model, this `Scene` is built once (or updated
/// incrementally) and then passed to the rendering system, which is responsible
/// for drawing it on every `Paint` event.
pub struct Scene {
    /// A vector of heap-allocated, dynamically-dispatched drawable objects.
    /// Using `Box<dyn Drawable>` allows the `Scene` to store any type that
    /// implements the `Drawable` trait.
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
    /// The object is boxed and added to the scene's list of `Drawable` trait
    /// objects. The rendering order is determined by the insertion order.
    ///
    /// # Type Parameters
    ///
    /// * `T`: Any type that implements `Drawable` and has a `'static` lifetime.
    ///
    /// # Arguments
    ///
    /// * `object`: The drawable object to add to the scene.
    pub fn add_object<T: Drawable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    /// Draws all objects in the scene using the provided `Renderer`.
    ///
    /// This method iterates through all the `Drawable` objects in the scene in
    /// the order they were added and calls their respective `draw` methods.
    ///
    /// # Arguments
    ///
    /// * `renderer`: A mutable reference to the `Renderer` used for drawing.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the underlying `draw` calls
    /// fail. The iteration will stop at the first error encountered.
    pub fn draw_all(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        for object in &self.objects {
            object.draw(renderer)?;
        }
        Ok(())
    }
}

impl Default for Scene {
    /// Creates a default, empty `Scene`, equivalent to `Scene::new()`.
    fn default() -> Self {
        Self::new()
    }
}