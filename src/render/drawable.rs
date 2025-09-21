
use crate::render::drawing_context::DrawingContext;
use windows::core::Result;

pub trait Drawable {
    fn draw(&self, context: &DrawingContext) -> Result<()>;
}
