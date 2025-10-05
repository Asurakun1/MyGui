//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use env_logger;
use my_gui::{core::render::objects::text_object::TextObject, prelude::*};

// 1. Define the application state.
pub struct App {
    pub scene: Scene,
    pub display_text: String,
    pub input_context: InputContext,
}

impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
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
        let display_text = "日本語ハローワールドテスト。".to_string();
        let mut scene = Scene::new();
        scene.add_object(TextObject::new(
            display_text.clone(),
            10.0,
            10.0,
            Color::GREEN,
        ));
        scene.add_object(Rectangle::new(10.0, 50.0, 200.0, 100.0, Color::WHITE));
        // Add a circle
        scene.add_object(Ellipse::new(300.0, 100.0, 50.0, 50.0, Color::WHITE));
        // Add a stretched ellipse
        scene.add_object(Ellipse::new(500.0, 100.0, 100.0, 30.0, Color::GREEN));
        // Add a line
        scene.add_object(Line::new(10.0, 170.0, 600.0, 200.0, 2.0, Color::GREEN));

        // Create a canvas
        let mut canvas = Canvas::new(250.0, 250.0, 300.0, 200.0);
        canvas.add_object(Rectangle::new(10.0, 10.0, 100.0, 50.0, Color::RED));
        canvas.add_object(Ellipse::new(150.0, 100.0, 50.0, 50.0, Color::BLUE));
        scene.add_object(canvas);

        Self {
            scene,
            display_text,
            input_context: InputContext::default(),
        }
    }
}

// 2. Define a custom event handler to print key events.
struct CustomEventHandler;

impl EventHandler<App> for CustomEventHandler {
    fn on_event(&mut self, app: &mut App, event: &Event, _renderer: &mut dyn Renderer) {
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                log::info!(
                    "KeyDown: {:?}, Modifiers: {:?}",
                    key,
                    app.input_context().keyboard
                );
            }
            Event::KeyUp(KeyboardEvent { key }) => {
                log::info!(
                    "KeyUp: {:?}, Modifiers: {:?}",
                    key,
                    app.input_context().keyboard
                );
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

// ... (rest of the file is the same until main)

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Hello, World!");
    // Now returns anyhow::Result
    let app = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));
    event_handler.add_handler(Box::new(CustomEventHandler)); // Add the custom handler

    let window = WindowBuilder::new()
        .with_title("Hello, World!")
        .with_width(900)
        .with_height(600)
        .build(event_handler, app)?;

    window.run()
}
