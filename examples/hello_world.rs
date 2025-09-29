//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use windows::core::*;


use my_gui::core::{
    backend::renderer::RendererConfig, // Import RendererConfig
    event::{render_event_handler::RenderEventHandler, root_event_handler::RootEventHandler},
    render::{
        objects::{primitives::{ellipse::Ellipse, line::Line, rectangle::Rectangle}, text_object::TextObject},
        scene::{HasScene, Scene},
    },
    window::{config::WindowConfig, WindowBuilder},
};

// 1. Define the application state.
pub struct App {
    pub scene: Scene,
    pub display_text: String,
}

impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }
}

impl App {
    pub fn new() -> Self {
        let display_text = "日本語ハローワールドテスト。".to_string();
        let mut scene = Scene::new();
        scene.add_object(TextObject::new(&display_text, 10.0, 10.0));
        scene.add_object(Rectangle::new(10.0, 50.0, 200.0, 100.0));
        // Add a circle
        scene.add_object(Ellipse::new(300.0, 100.0, 50.0, 50.0));
        // Add a stretched ellipse
        scene.add_object(Ellipse::new(500.0, 100.0, 100.0, 30.0));
        // Add a line
        scene.add_object(Line::new_with_xy(10.0, 170.0, 600.0, 200.0, 2.0));
        Self {
            scene,
            display_text,
        }
    }
}

fn main() -> Result<()> {
    let app = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(RenderEventHandler::new()));

    let config = WindowConfig {
        title: "Hello, World!".to_string(),
        width: 900,
        height: 600,
        renderer_config: RendererConfig::Direct2D, // Specify the renderer
        ..Default::default()
    };

    let window = WindowBuilder::from_config(config).build(event_handler, app)?;

    let result = window.run();

    std::mem::forget(window);

    result
}