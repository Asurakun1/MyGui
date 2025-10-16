use my_gui_core::prelude::*;
use taffy::prelude::*;
use my_gui_core::prelude::{BaseWidget, Widget};

pub struct RectangleWidget {
    base: BaseWidget,
    rectangle: Rectangle,
}

impl RectangleWidget {
    pub fn new(id: usize, style: Style, color: Color, border_color: Option<Color>, border_thickness: Option<f32>) -> Self {
        let rectangle = Rectangle::new(0.0, 0.0, 0.0, 0.0, color, border_color, border_thickness);
        let base = BaseWidget::new(id, style);
        Self { base, rectangle }
    }
}

impl Widget for RectangleWidget {
    fn get_style(&self) -> &Style {
        self.base.get_style()
    }

    fn set_style(&mut self, style: Style) {
        self.base.set_style(style);
    }

    fn get_layout_node(&self) -> Option<NodeId> {
        self.base.get_layout_node()
    }

    fn set_layout_node(&mut self, node_id: NodeId) {
        self.base.set_layout_node(node_id);
    }

    fn update_layout(&mut self, layout: &taffy::Layout) {
        self.rectangle.x = layout.location.x;
        self.rectangle.y = layout.location.y;
        self.rectangle.width = layout.size.width;
        self.rectangle.height = layout.size.height;
    }

    fn handle_event(&mut self, event: &Event) -> EventResult {
        self.base.handle_event(event)
    }

    fn as_drawable(&self) -> &dyn Drawable {
        self
    }

    fn as_drawable_mut(&mut self) -> &mut dyn Drawable {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Drawable for RectangleWidget {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        renderer.draw_rectangle(&self.rectangle)?;
        if self.rectangle.border_color.is_some() && self.rectangle.border_thickness.is_some() {
            renderer.draw_rectangle_border(&self.rectangle)?;
        }
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct RectangleWidgetBuilder {
    id: usize,
    style: Style,
    color: Color,
    border_color: Option<Color>,
    border_thickness: Option<f32>,
}

impl RectangleWidgetBuilder {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            style: Style::default(),
            color: Color::BLACK,
            border_color: None,
            border_thickness: None,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn with_border_thickness(mut self, border_thickness: f32) -> Self {
        self.border_thickness = Some(border_thickness);
        self
    }

    pub fn build(self) -> RectangleWidget {
        RectangleWidget::new(
            self.id,
            self.style,
            self.color,
            self.border_color,
            self.border_thickness,
        )
    }
}
