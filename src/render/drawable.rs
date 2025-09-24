
use crate::render::drawing_context::DrawingContext;
use windows::core::Result;

/// A trait for objects that can be drawn to a `DrawingContext`.
///
/// This trait provides a common interface for all drawable objects in the scene.
pub trait Drawable {
    /// Draws the object to the given `DrawingContext`.
    ///
    /// # Arguments
    ///
    /// * `context` - The `DrawingContext` to draw to.
    fn draw(&self, context: &DrawingContext) -> Result<()>;
}
