//! # MyGui Hello Taffy
//!
//! This is a simple example of how to use the `my_gui` framework with the
//! `taffy` crate to create a simple layout.
use env_logger;
use my_gui::prelude::*;
use taffy::prelude::*;

// 1. Define the application state.
pub struct App {
    pub scene: Scene,
    pub input_context: InputContext,
    pub taffy: TaffyTree,
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
        let mut taffy = TaffyTree::new();

        // Create a root node that fills the entire window
        let root_style = Style {
            size: Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            },
            justify_content: Some(JustifyContent::Center), // Center children horizontally
            align_items: Some(AlignItems::Center),     // Center children vertically
            ..Default::default()
        };

        // Create a child node with a fixed size
        let child_style = Style {
            size: Size {
                width: Dimension::length(200.0),
                height: Dimension::length(100.0),
            },
            ..Default::default()
        };

        let child = taffy.new_leaf(child_style).unwrap();
        let root = taffy.new_with_children(root_style, &[child]).unwrap();

        // Compute the layout
        taffy
            .compute_layout(
                root,
                Size { width: AvailableSpace::Definite(900.0), height: AvailableSpace::Definite(600.0) },
            )
            .unwrap();

        let child_layout = taffy.layout(child).unwrap();

        // Create a rectangle drawable and position it using the computed layout
        let rect = Rectangle::new(
            child_layout.location.x,
            child_layout.location.y,
            child_layout.size.width,
            child_layout.size.height,
            Color::BLUE,
        );

        let mut scene = Scene::new();
        scene.add_object(rect);

        Self {
            scene,
            input_context: InputContext::default(),
            taffy,
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

    let window = WindowBuilder::new()
        .with_title("Hello, Taffy!")
        .with_width(900)
        .with_height(600)
        .build(event_handler, app)?;

    window.run()
}
