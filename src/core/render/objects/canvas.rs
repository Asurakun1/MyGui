use crate::core::render::drawable::Drawable;
use crate::core::backend::renderer::Renderer;
use anyhow::Result;

pub struct Canvas {
    objects: Vec<Box<dyn Drawable>>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Canvas {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            objects: Vec::new(),
            x,
            y,
            width,
            height,
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    pub fn set_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

use glam::Affine2;

impl Drawable for Canvas {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        let original_transform = renderer.get_transform();

        let translation = Affine2::from_translation(glam::vec2(self.x, self.y));

        renderer.set_transform(&translation);

        renderer.push_axis_aligned_clip(0.0, 0.0, self.width, self.height);

        for object in &self.objects {
            object.draw(renderer)?;
        }

        renderer.pop_axis_aligned_clip();

        renderer.set_transform(&original_transform);

        Ok(())
    }
}
