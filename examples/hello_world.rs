//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
//use windows::core::*;

use std::error::Error;

use my_gui::core::{
    event::{render_event_handler::RenderEventHandler, root_event_handler::RootEventHandler},
    render::{
        objects::text_object::TextObject,
        scene::{HasScene, Scene},
    },
    window::{WindowBuilder, config::WindowConfig},
};

// 1. Define the application state.
// The user of the library is responsible for defining the application state.
// This struct can contain anything the application needs to store.
pub struct App {
    /// The scene containing all drawable objects.
    pub scene: Scene,
    /// The text string to be displayed in the window.
    pub display_text: String,
}

// 2. Implement the `HasScene` trait for the application state.
// This allows the `RenderEventHandler` to access the scene.
impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }
}

impl App {
    /// Creates a new `App` instance with a default scene.
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

fn main() -> Result<(), Box<dyn Error>> {
    // Create the application state.
    let app = App::new();

    // Create the event handlers.
    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(RenderEventHandler::new()));

    // Create the window configuration.
    let config = WindowConfig {
        title: "Hello, World!".to_string(),
        width: 900,
        height: 600,
        ..Default::default()
    };

    // Create the window.
    let window = WindowBuilder::from_config(config).build(event_handler, app)?;

    // Run the application.
    let result = window.run();

    // The window is intentionally "leaked" using `std::mem::forget` because its
    // lifetime is managed by the Windows API.
    std::mem::forget(window);

    result?;

    Ok(())
}
