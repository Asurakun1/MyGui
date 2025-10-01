//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use anyhow::Result; // Use anyhow::Result

use my_gui::core::{
    backend::renderer::Renderer,
    backend::renderer::RendererConfig, // Import RendererConfig
    event::{
        Event,
        event_handler::EventHandler,
        handlers::{
            keyboard_handler::{KeyboardEvent, KeyboardInputHandler},
            mouse_handler::{HasMouseState, MouseEvent, MouseInputHandler, MouseState},
            render_event_handler::RenderEventHandler,
            root_event_handler::RootEventHandler,
        },
        input_state::{HasInputState, InputState},
    },
    render::{
        objects::{
            primitives::{ellipse::Ellipse, line::Line, rectangle::Rectangle},
            text_object::TextObject,
        },
        scene::{HasScene, Scene},
    },
    window::{WindowBuilder, config::KeyboardInputMode, config::WindowConfig},
};

// 1. Define the application state.
pub struct App {
    pub scene: Scene,
    pub display_text: String,
    pub input_state: InputState,
    pub mouse_state: MouseState,
}

impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }
}

impl HasInputState for App {
    fn input_state(&self) -> &InputState {
        &self.input_state
    }

    fn input_state_mut(&mut self) -> &mut InputState {
        &mut self.input_state
    }
}

impl HasMouseState for App {
    fn mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    fn mouse_state_mut(&mut self) -> &mut MouseState {
        &mut self.mouse_state
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
            input_state: InputState::default(),
            mouse_state: MouseState::default(),
        }
    }
}

// 2. Define a custom event handler to print key events.
struct CustomEventHandler;

impl EventHandler<App> for CustomEventHandler {
    fn on_event(&mut self, app: &mut App, event: &Event, _renderer: &mut dyn Renderer) {
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                println!("KeyDown: {:?}, Modifiers: {:?}", key, app.input_state());
            }
            Event::KeyUp(KeyboardEvent { key }) => {
                println!("KeyUp: {:?}, Modifiers: {:?}", key, app.input_state());
            }
            // Event::MouseMove(MouseEvent { x, y, .. }) => {
            //     println!("MouseMove: x: {}, y: {}", x, y);
            // }
            Event::MouseDown(MouseEvent { button, .. }) => {
                println!("MouseDown: {:?}", button);
            }
            Event::MouseUp(MouseEvent { button, .. }) => {
                println!("MouseUp: {:?}", button);
            }
            Event::MouseWheel(delta) => {
                println!("MouseWheel: {:?}", delta);
            }
            Event::Character(character) => {
                println!("Character: {}", character);
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    // Now returns anyhow::Result
    let app = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(RenderEventHandler::new()));
    event_handler.add_handler(Box::new(KeyboardInputHandler::default()));
    event_handler.add_handler(Box::new(MouseInputHandler));
    event_handler.add_handler(Box::new(CustomEventHandler)); // Add the custom handler

    let config = WindowConfig {
        title: "Hello, World!".to_string(),
        width: 900,
        height: 600,
        renderer_config: RendererConfig::Direct2D, // Specify the renderer
        keyboard_input_mode: KeyboardInputMode::Translated,
        ..Default::default()
    };

    let window = WindowBuilder::from_config(config).build(event_handler, app)?;

    let result = window.run();

    std::mem::forget(window);

    result
}
