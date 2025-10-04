//! # Ellipse Primitive
//!
//! This module defines the `Ellipse` struct, a `Drawable` primitive for rendering
//! a solid-color ellipse or circle.

use crate::core::{
    backend::renderer::Renderer,
    render::{color::Color, drawable::Drawable},
};

/// A `Drawable` struct that represents a filled ellipse.
///
/// This struct defines an ellipse by its `center_x` and `center_y` coordinates,
/// its horizontal (`radius_x`) and vertical (`radius_y`) radii, and its fill `color`.
/// To define a circle, simply set `radius_x` and `radius_y` to the same value.
///
/// Like other primitives, this struct is a simple data container that delegates
/// the rendering logic to the `draw_ellipse` method of a [`Renderer`].
pub struct Ellipse {
    /// The x-coordinate of the center of the ellipse.
    pub center_x: f32,
    /// The y-coordinate of the center of the ellipse.
    pub center_y: f32,
    /// The radius of the ellipse along the x-axis.
    pub radius_x: f32,
    /// The radius of the ellipse along the y-axis.
    pub radius_y: f32,
    /// The fill color of the ellipse.
    pub color: Color,
}

impl Ellipse {
    /// Creates a new `Ellipse` with the specified center, radii, and color.
    ///
    /// # Arguments
    ///
    /// * `center_x` - The x-coordinate of the ellipse's center.
    /// * `center_y` - The y-coordinate of the ellipse's center.
    /// * `radius_x` - The horizontal radius of the ellipse.
    /// * `radius_y` - The vertical radius of the ellipse.
    /// * `color` - The `Color` to fill the ellipse with.
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
    /// Draws the ellipse by delegating to the active `Renderer`.
    ///
    /// This method calls the `draw_ellipse` method on the provided `Renderer`,
    /// passing a reference to itself.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` that will perform the drawing operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer's `draw_ellipse`
    /// method fails.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_ellipse(self)
    }
}