//! # MyGui Hello Taffy
//!
//! This is a simple example of how to use the `my_gui` framework with the
//! `taffy` crate to create a simple layout.
use env_logger;
use my_gui::core::event::handlers::layout_event_handler::LayoutEventHandler;
use my_gui::prelude::*;
use taffy::prelude::*;

// 1. Define the application state.
pub struct App {
    pub input_context: InputContext,
    pub scene: Scene,
}

impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

impl App {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();

        // Create a root node that fills the entire window
        let root_style = Style {
            size: Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            },
            justify_content: Some(JustifyContent::Center), // Center children horizontally
            align_items: Some(AlignItems::Center),         // Center children vertically
            ..Default::default()
        };

        let root_node = taffy.new_leaf(root_style).unwrap();

        let mut root = LayoutNode {
            taffy_node: root_node,
            drawable: Some(Box::new(Rectangle::new(0.0, 0.0, 0.0, 0.0, Color::BLACK))), // Placeholder
            children: vec![],
        };

        // Create a child node with a fixed size
        let child_style = Style {
            size: Size {
                width: Dimension::length(200.0),
                height: Dimension::length(100.0),
            },
            ..Default::default()
        };

        let child_node = taffy.new_leaf(child_style).unwrap();
        let child = LayoutNode {
            taffy_node: child_node,
            drawable: Some(Box::new(Rectangle::new(
                0.0,
                0.0,
                200.0,
                100.0,
                Color::BLUE,
            ))),
            children: vec![],
        };

        taffy.add_child(root.taffy_node, child.taffy_node).unwrap();
        root.children.push(child);

        let layout_tree = LayoutTree {
            taffy,
            root: Some(root),
            is_dirty: true,
        };

        let scene = Scene {
            layout_tree,
        };

        Self {
            input_context: InputContext::default(),
            scene,
        }
    }
}

fn main() -> Result<()> {
    let _app = Application::new()?;
    env_logger::init();
    log::info!("Hello, Taffy!");

    let app = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));
    event_handler.add_handler(Box::new(LayoutEventHandler));

    let window = WindowBuilder::new()
        .with_title("Hello, Taffy!")
        .with_width(900)
        .with_height(600)
        .build(event_handler, app)?;

    window.run()
}
