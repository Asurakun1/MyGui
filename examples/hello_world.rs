//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use env_logger;
use my_gui::prelude::*;

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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let display_text = "日本語ハローワールドテスト。".to_string();
        let mut scene = Scene::new();
        // scene.add_object(TextObject::new(&display_text, 10.0, 10.0));
        // scene.add_object(Rectangle::new(10.0, 50.0, 200.0, 100.0));
        // // Add a circle
        // scene.add_object(Ellipse::new(300.0, 100.0, 50.0, 50.0));
        // // Add a stretched ellipse
        // scene.add_object(Ellipse::new(500.0, 100.0, 100.0, 30.0));
        // // Add a line
        // scene.add_object(Line::new_with_xy(10.0, 170.0, 600.0, 200.0, 2.0));

        // Create a canvas
        let mut canvas = Canvas::new(250.0, 250.0, 300.0, 200.0);
        canvas.add_object(Box::new(Rectangle::new(
            10.0,
            10.0,
            100.0,
            50.0,
            Color::RED,
        )));
        canvas.add_object(Box::new(Ellipse::new(
            150.0,
            100.0,
            50.0,
            50.0,
            Color::BLUE,
        )));
        scene.add_object(canvas);

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
                log::info!("KeyDown: {:?}, Modifiers: {:?}", key, app.input_state());
            }
            Event::KeyUp(KeyboardEvent { key }) => {
                log::info!("KeyUp: {:?}, Modifiers: {:?}", key, app.input_state());
            }
            // Event::MouseMove(MouseEvent { x, y, .. }) => {
            //     log::info!("MouseMove: x: {}, y: {}", x, y);
            // }
            Event::MouseDown(MouseEvent { button, .. }) => {
                log::info!("MouseDown: {:?}", button);
            }
            Event::MouseUp(MouseEvent { button, .. }) => {
                log::info!("MouseUp: {:?}", button);
            }
            Event::MouseWheel(delta) => {
                log::info!("MouseWheel: {:?}", delta);
            }
            Event::Character(character) => {
                log::info!("Character: {}", character);
            }
            _ => {}
        }
    }
}

use my_gui::core::event::event_loop::EventLoop;

// ... (rest of the file is the same until main)

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Hello, World!");
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
        renderer_config: RendererConfig::Direct2D(Default::default()), // Specify the renderer
        keyboard_input_mode: KeyboardInputMode::Translated,
        ..Default::default()
    };

    let window = Window::new(config, event_handler, app)?;

    let mut event_loop = EventLoop::new();
    event_loop.run()?;

    std::mem::forget(window);

    Ok(())
}
