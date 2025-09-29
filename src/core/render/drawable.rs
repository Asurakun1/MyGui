
use crate::core::backend::renderer::Renderer;
use windows::core::Result;

/// A trait for objects that can be drawn to a `Renderer`.
///
/// This trait defines the core contract for any object that can be rendered on the screen.
/// By implementing `Drawable`, a struct can be added to a `Scene` and participate in the
/// rendering pipeline.
pub trait Drawable {
    /// Draws the object to the given `Renderer`.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The `Renderer` to draw to.
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()>;
}
