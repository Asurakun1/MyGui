
use crate::core::render::drawing_context::DrawingContext;
use windows::core::Result;

/// A trait for objects that can be drawn to a `DrawingContext`.
///
/// This trait defines the core contract for any object that can be rendered on the screen.
/// By implementing `Drawable`, a struct can be added to a `Scene` and participate in the
/// rendering pipeline. This abstraction allows the `Scene` to manage a heterogeneous
/// collection of different drawable types (e.g., text, shapes, images) without knowing
/// their concrete implementations.
pub trait Drawable {
    /// Draws the object to the given `DrawingContext`.
    ///
    /// # Arguments
    ///
    /// * `context` - The `DrawingContext` to draw to.
    fn draw(&self, context: &DrawingContext) -> Result<()>;
}
