use my_gui::prelude::*;
use my_gui_widgets::layout::Layout as MyGuiLayout; // Re-add this line
use my_gui_widgets::rectangle_widget::RectangleWidget;
use my_gui_widgets::widget_scene::WidgetScene;
use taffy::prelude::*; // Re-add this line

// 1. Define the application state.
pub struct App {
    pub widget_scene: my_gui_widgets::widget_scene::WidgetScene,
    pub input_context: InputContext,
    pub layout: my_gui_widgets::layout::Layout,
}

impl my_gui_core::prelude::HasDrawableCollection for App {
    fn draw_all_drawables(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        self.widget_scene.draw_all_drawables(renderer)
    }
}

impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let mut layout = MyGuiLayout::new(Style {
            size: Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            },
            justify_content: Some(JustifyContent::Center), // Center children horizontally
            align_items: Some(AlignItems::Center),         // Center children vertically
            ..Default::default()
        });
        let mut widget_scene = WidgetScene::new();

        for i in 0..4 {
            let rect_widget = RectangleWidget::new(
                i,
                Style {
                    size: Size {
                        width: Dimension::percent(0.2),
                        height: Dimension::percent(0.2),
                    },
                    border: Rect {
                        left: LengthPercentage::length(10.0),
                        right: LengthPercentage::length(10.0),
                        top: LengthPercentage::length(10.0),
                        bottom: LengthPercentage::length(10.0),
                    },
                    padding: Rect {
                        left: LengthPercentage::length(10.0),
                        right: LengthPercentage::length(10.0),
                        top: LengthPercentage::length(10.0),
                        bottom: LengthPercentage::length(10.0),
                    },
                    ..Default::default()
                },
                match i {
                    0 => Color::RED,
                    1 => Color::GREEN,
                    2 => Color::BLUE,
                    3 => Color::YELLOW,
                    _ => Color::BLACK,
                },
                Some(Color::WHITE),
                Some(10.0),
            );

            let node_id = layout.add_widget(i, rect_widget.get_style().clone(), &[]);
            let mut boxed_widget = Box::new(rect_widget);
            boxed_widget.set_layout_node(node_id);
            widget_scene.add_widget(boxed_widget);
        }

        layout.compute_layout(Size {
            width: AvailableSpace::Definite(900.0),
            height: AvailableSpace::Definite(600.0),
        });

        widget_scene.update_widget_layouts(&layout);

        Self {
            widget_scene,
            input_context: InputContext::default(),
            layout,
        }
    }
}
