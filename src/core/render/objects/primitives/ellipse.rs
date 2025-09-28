use crate::core::render::drawable::Drawable;
use crate::core::render::drawing_context::DrawingContext;
use windows::core::Result;
use windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE;
use windows_numerics::Vector2;

/// A `Drawable` object that represents an ellipse.
pub struct Ellipse {
    pub center: Vector2,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl Ellipse {
    /// Creates a new `Ellipse` with the specified center and radii.
    pub fn new(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center: Vector2 { X: center_x, Y: center_y },
            radius_x,
            radius_y,
        }
    }
}

impl Drawable for Ellipse {
    /// Draws the ellipse to the render target using the provided `DrawingContext`.
    ///
    /// # Safety
    ///
    /// This function contains an `unsafe` block for calling the Direct2D `FillEllipse`
    /// method. The caller must ensure that the `drawing_context` contains valid
    /// Direct2D resources.
    fn draw(&self, context: &DrawingContext) -> Result<()> {
        let ellipse = D2D1_ELLIPSE {
            point: self.center.into(),
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        };

        unsafe {
            context.render_target.FillEllipse(&ellipse, context.brush);
        }

        Ok(())
    }
}