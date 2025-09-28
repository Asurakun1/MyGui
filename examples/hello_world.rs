//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use windows::core::*;

use my_gui::core::{
    event::{render_event_handler::RenderEventHandler, root_event_handler::RootEventHandler},
    platform::window_backend::WindowBackend, // Add this line
    render::{
        objects::text_object::TextObject,
        scene::{HasScene, Scene},
    },
    window::{WindowBuilder, config::WindowConfig},
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
        scene.add_object(Box::new(TextObject::new(&display_text, 10.0, 10.0)));
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
        ..Default::default()
    };

    let window = WindowBuilder::from_config(config).build(event_handler, app)?;

    let result = window.run();

    std::mem::forget(window);

    result
}
