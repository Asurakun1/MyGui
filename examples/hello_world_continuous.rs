//! # MyGui Hello World - Continuous Mode
//!
//! This example demonstrates how to use the `Continuous` run mode to create
//! an application with a fixed-timestep game loop.

use my_gui::{
    core::{render::objects::text_object::TextObject, window::config::RunMode},
    prelude::*,
};
use std::time::Duration;

// 1. Define the application state.
pub struct App {
    scene: Scene,
    input_context: InputContext,
    frame_count: u64,
    last_fps_update: Duration,
    // We hold onto the index of the text object in the scene graph
    // so we can modify it later.
    fps_text_object_index: usize,
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

impl App {
    pub fn new() -> Self {
        let mut scene = Scene::new();
        // Add a static text object
        scene.add_object(TextObject::new(
            "Hello, World! This is a continuous loop.".to_string(),
            10.0,
            10.0,
            Color::GREEN,
        ));

        // Add a text object for our FPS counter.
        let fps_text = TextObject::new("Frame: 0".to_string(), 10.0, 40.0, Color::WHITE);
        // When we add it, we get its index in the scene graph back.
        let fps_text_object_index = scene.add_object(fps_text);

        Self {
            scene,
            input_context: InputContext::default(),
            frame_count: 0,
            last_fps_update: Duration::from_secs(0),
            fps_text_object_index,
        }
    }
}

// 2. Define a custom event handler.
struct CustomEventHandler;

impl EventHandler<App> for CustomEventHandler {
    fn on_event(
        &mut self,
        app: &mut App,
        event: &Event,
        _renderer: &mut dyn Renderer,
    ) -> EventResult {
        match event {
            // The new Update event, dispatched on every frame in continuous mode.
            Event::Update(delta_time) => {
                app.frame_count += 1;
                app.last_fps_update += *delta_time;

                // Update the FPS counter text every second.
                if app.last_fps_update.as_secs() >= 1 {
                    // Copy the data we need before creating a mutable borrow.
                    let frame_count = app.frame_count;
                    let index = app.fps_text_object_index;

                    // Get a mutable reference to the text object from the scene.
                    if let Some(text_object) = app
                        .scene_mut()
                        .get_object_mut(index)
                        .and_then(|o| o.as_any_mut().downcast_mut::<TextObject>())
                    {
                        text_object.set_text(format!("Frame: {}", frame_count));
                    }
                    //app.last_fps_update = Duration::from_secs(0);
                }
            }
            Event::WindowClose => {
                println!("Bye Bye!");
            }
            _ => {}
        }
        // We don't consume the events, so other handlers could see them if they existed.
        EventResult::NotConsumed
    }
}

fn main() -> Result<()> {
    let _app = Application::new()?;
    let app_state = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));
    event_handler.add_handler(Box::new(CustomEventHandler));

    let window = WindowBuilder::new()
        .with_title("Hello, World! (Continuous Mode @ 60 FPS)")
        .with_width(900)
        .with_height(600)
        // This is the key change to enable the game loop!
        .with_run_mode(RunMode::Continuous { target_fps: 60 })
        .build(event_handler, app_state)?;

    window.run()
}
