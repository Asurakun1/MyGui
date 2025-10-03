//! # Ellipse Primitive
//!
//! This module defines the `Ellipse` struct, a `Drawable` primitive for rendering
//! solid-color ellipses and circles.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;

/// A `Drawable` struct that represents an ellipse.
///
/// This struct defines an ellipse by its center point (`center_x`, `center_y`) and
/// its horizontal and vertical radii (`radius_x`, `radius_y`). To draw a circle,
/// simply set `radius_x` and `radius_y` to the same value.
///
/// Like other primitives, this struct is a simple data container that delegates
/// the rendering work to the `Renderer`'s `draw_ellipse` method.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::render::objects::primitives::Ellipse;
/// use my_gui::core::render::scene::Scene;
///
/// // Create an ellipse centered at (100, 100) with a horizontal radius
/// // of 50 and a vertical radius of 30.
/// let ellipse = Ellipse::new(100.0, 100.0, 50.0, 30.0);
///
/// // Add it to a scene to be rendered.
/// let mut scene = Scene::new();
/// scene.add_object(ellipse);
/// ```
pub struct Ellipse {
    /// The x-coordinate of the center of the ellipse.
    pub center_x: f32,
    /// The y-coordinate of the center of the ellipse.
    pub center_y: f32,
    /// The radius of the ellipse along the x-axis.
    pub radius_x: f32,
    /// The radius of the ellipse along the y-axis.
    pub radius_y: f32,
}

impl Ellipse {
    /// Creates a new `Ellipse` with the specified center and radii.
    pub fn new(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center_x,
            center_y,
            radius_x,
            radius_y,
        }
    }
}

impl Drawable for Ellipse {
    /// Draws the ellipse by delegating to the `Renderer`.
    ///
    /// This method calls the `draw_ellipse` method on the provided `Renderer`,
    /// passing the necessary geometric data.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_ellipse` method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_ellipse(self)
    }
}