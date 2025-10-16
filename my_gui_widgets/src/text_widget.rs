use my_gui_core::core::render::objects::text_object::TextObject;
use my_gui_core::prelude::*;
use my_gui_core::prelude::{BaseWidget, Widget};
use taffy::prelude::*;

pub struct TextWidget {
    base: BaseWidget,
    text_object: TextObject,
}

impl TextWidget {
    pub fn new(id: usize, style: Style, text: String, font_size: f32, color: Color) -> Self {
        let text_object = TextObject::new(
            text,
            0.0, // Initial x
            0.0, // Initial y
            0.0, // Initial width
            0.0, // Initial height
            font_size,
            color,
        );
        let base = BaseWidget::new(id, style);
        Self { base, text_object }
    }
}

impl Widget for TextWidget {
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
        self.text_object.x = layout.location.x;
        self.text_object.y = layout.location.y;
        self.text_object.width = layout.size.width;
        self.text_object.height = layout.size.height;
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

impl Drawable for TextWidget {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        if self.text_object.layout.is_none() {
            self.text_object.layout = Some(renderer.create_text_layout(&self.text_object)?);
        }
        renderer.draw_text_layout(&self.text_object)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct TextWidgetBuilder {
    id: usize,
    style: Style,
    text: String,
    font_size: f32,
    color: Color,
}

impl TextWidgetBuilder {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            style: Style::default(),
            text,
            font_size: 12.0, // Default font size
            color: Color::BLACK,
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

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn build(self) -> TextWidget {
        TextWidget::new(self.id, self.style, self.text, self.font_size, self.color)
    }
}
