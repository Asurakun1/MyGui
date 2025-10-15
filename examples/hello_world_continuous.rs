//! # MyGui Hello World - Continuous Mode
//!
//! This example demonstrates how to use the `Continuous` run mode to create
//! an application with a fixed-timestep game loop.

use my_gui::{core::window::config::RunMode, prelude::*};
use std::time::Duration;
use taffy::prelude::*;

// 1. Define the application state.
pub struct App {
    scene: Scene,
    input_context: InputContext,
    frame_count: u64,
    last_fps_update: Duration,
    fps_text_node_id: taffy::NodeId,
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
        let mut taffy = TaffyTree::new();

        let root_style = Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        };
        let root_node = taffy.new_leaf(root_style).unwrap();

        let mut root = LayoutNode {
            taffy_node: root_node,
            drawable: None,
            children: vec![],
        };

        let text_style = Style {
            size: Size { width: Dimension::length(400.0), height: Dimension::length(20.0) },
            ..Default::default()
        };
        let text_node = taffy.new_leaf(text_style).unwrap();
        let text = LayoutNode {
            taffy_node: text_node,
            drawable: Some(Box::new(TextObject::new("Hello, World! This is a continuous loop.".to_string(), 0.0, 0.0, 400.0, 20.0, Color::GREEN))),
            children: vec![],
        };
        root.children.push(text);
        taffy.add_child(root.taffy_node, text_node).unwrap();

        let fps_text_style = Style {
            size: Size { width: Dimension::length(200.0), height: Dimension::length(20.0) },
            ..Default::default()
        };
        let fps_text_node = taffy.new_leaf(fps_text_style).unwrap();
        let fps_text = LayoutNode {
            taffy_node: fps_text_node,
            drawable: Some(Box::new(TextObject::new("Frame: 0".to_string(), 0.0, 0.0, 200.0, 20.0, Color::WHITE))),
            children: vec![],
        };
        root.children.push(fps_text);
        taffy.add_child(root.taffy_node, fps_text_node).unwrap();

        let scene = Scene {
            layout_tree: LayoutTree {
                taffy,
                root: Some(root),
                is_dirty: true,
            },
        };

        Self {
            scene,
            input_context: InputContext::default(),
            frame_count: 0,
            last_fps_update: Duration::from_secs(0),
            fps_text_node_id: fps_text_node,
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
                    let node_id = app.fps_text_node_id;

                    // Get a mutable reference to the text object from the layout tree.
                    if let Some(text_object) = app
                        .scene_mut()
                        .layout_tree
                        .get_node_mut(node_id)
                        .and_then(|n| n.drawable.as_mut())
                        .and_then(|d| d.as_any_mut().downcast_mut::<TextObject>())
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
    event_handler.add_handler(Box::new(LayoutEventHandler));

    let window = WindowBuilder::new()
        .with_title("Hello, World! (Continuous Mode @ 60 FPS)")
        .with_width(900)
        .with_height(600)
        // This is the key change to enable the game loop!
        .with_run_mode(RunMode::Continuous { target_fps: 60 })
        .build(event_handler, app_state)?;

    window.run()
}
