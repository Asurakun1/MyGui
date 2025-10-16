use my_gui::prelude::*;
use taffy::prelude::*;

// 1. Define the application state.
pub struct App {
    pub scene: Scene,
    pub input_context: InputContext,
    pub layout: my_gui_widgets::layout::Layout,
}

impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
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
        use my_gui_widgets::layout::Layout as MyGuiLayout;
        use taffy::prelude::*;

        let root_style = Style {
            size: Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            },
            justify_content: Some(JustifyContent::Center), // Center children horizontally
            align_items: Some(AlignItems::Center),         // Center children vertically
            ..Default::default()
        };

        let child_style = Style {
            size: Size {
                width: Dimension::percent(0.6),
                height: Dimension::percent(0.6),
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
        };

        let mut layout = MyGuiLayout::new(root_style);
        let child_style_for_first_widget = child_style.clone();
        let _child_node_id = layout.add_widget(0, child_style_for_first_widget, &[]); // Assuming widget_id 0 for the child
        layout.add_widget(1, child_style.clone(), &[]); // Another child
        layout.add_widget(2, child_style.clone(), &[]); // Another child

        layout.compute_layout(Size {
            width: AvailableSpace::Definite(900.0),
            height: AvailableSpace::Definite(600.0),
        });

        let mut child_layouts = Vec::new();
        for i in 0..3 {
            child_layouts.push(layout.get_layout(i).unwrap());
        }

        let scene = crate::draw_objects::create_scene_objects(&child_layouts);

        Self {
            scene,
            input_context: InputContext::default(),
            layout,
        }
    }
}
