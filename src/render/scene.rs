
use windows::core::Result;

use crate::render::drawable::Drawable;
use crate::render::drawing_context::DrawingContext;

pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    pub fn draw_all(&self, context: &DrawingContext) -> Result<()> {
        for object in &self.objects {
            object.draw(context)?;
        }
        Ok(())
    }
}
