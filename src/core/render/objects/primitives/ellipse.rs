//! # Ellipse Primitive
//!
//! This module defines the `Ellipse` struct, a `Drawable` primitive for rendering
//! solid-color ellipses and circles.

use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;
use crate::core::render::color::Color;

/// A `Drawable` struct that represents an ellipse.
///
/// This struct defines an ellipse by its center point (`center_x`, `center_y`),
/// its horizontal and vertical radii (`radius_x`, `radius_y`), and its `color`.
/// To draw a circle, simply set `radius_x` and `radius_y` to the same value.
///
/// Like other primitives, this struct is a simple data container that delegates
/// the rendering work to the `Renderer`'s `draw_ellipse` method.
pub struct Ellipse {
    /// The x-coordinate of the center of the ellipse.
    pub center_x: f32,
    /// The y-coordinate of the center of the ellipse.
    pub center_y: f32,
    /// The radius of the ellipse along the x-axis.
    pub radius_x: f32,
    /// The radius of the ellipse along the y-axis.
    pub radius_y: f32,
    /// The color of the ellipse, represented by a `Color` struct.
    pub color: Color,
}

impl Ellipse {
    /// Creates a new `Ellipse` with the specified center coordinates, radii, and color.
    ///
    /// # Arguments
    ///
    /// * `center_x` - The x-coordinate of the ellipse's center.
    /// * `center_y` - The y-coordinate of the ellipse's center.
    /// * `radius_x` - The horizontal radius of the ellipse.
    /// * `radius_y` - The vertical radius of the ellipse.
    /// * `color` - The `Color` of the ellipse.
    pub fn new(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32, color: Color) -> Self {
        Self {
            center_x,
            center_y,
            radius_x,
            radius_y,
            color,
        }
    }
}

impl Drawable for Ellipse {
    /// Draws the ellipse by delegating to the `Renderer`.
    ///
    /// This method calls the `draw_ellipse` method on the provided `Renderer`,
    /// passing itself (which contains all the necessary geometric and color data).
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