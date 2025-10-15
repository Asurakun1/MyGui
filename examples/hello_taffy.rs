//! # MyGui Hello Taffy
//!
//! This is a simple example demonstrating how to integrate the `taffy` layout
//! system with the `my_gui` framework. It showcases how to define UI elements
//! using `LayoutNode`s, apply `taffy` styles, and have the framework
//! automatically manage their positioning and sizing.

use env_logger;
use my_gui::core::event::handlers::layout_event_handler::LayoutEventHandler;
use my_gui::prelude::*;
use taffy::prelude::*;

/// The application's state, holding the input context and the scene graph.
pub struct App {
    pub input_context: InputContext,
    pub scene: Scene,
}

/// Implements `HasInputContext` to provide access to the input state.
impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

/// Implements `HasScene` to provide access to the application's scene graph.
impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

impl App {
    /// Creates a new `App` instance, setting up the `TaffyTree` and initial `LayoutNode`s.
    pub fn new() -> Self {
        // Initialize a new TaffyTree, which will manage our UI layout.
        let mut taffy = TaffyTree::new();

        // Define the style for the root node. This node will fill the entire window
        // and center its children both horizontally and vertically.
        let root_style = Style {
            size: Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            },
            justify_content: Some(JustifyContent::Center), // Center children horizontally
            align_items: Some(AlignItems::Center),         // Center children vertically
            ..Default::default()
        };

        // Create the root Taffy node with the defined style.
        let root_node = taffy.new_leaf(root_style).unwrap();

        // Create a `LayoutNode` for the root. This `LayoutNode` associates the
        // Taffy node with an optional `Drawable` object. Here, we use a placeholder
        // rectangle that will be sized and positioned by Taffy.
        let mut root = LayoutNode {
            taffy_node: root_node,
            drawable: Some(Box::new(Rectangle::new(0.0, 0.0, 0.0, 0.0, Color::BLACK))), // Placeholder
            children: vec![],
        };

        // Define the style for a child node. This child will have a fixed size.
        let child_style = Style {
            size: Size {
                width: Dimension::length(200.0),
                height: Dimension::length(100.0),
            },
            ..Default::default()
        };

        // Create the child Taffy node with its style.
        let child_node = taffy.new_leaf(child_style).unwrap();
        // Create a `LayoutNode` for the child, associating it with a blue rectangle.
        // Its position and size will be determined by Taffy.
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

        // Add the child Taffy node to the root Taffy node.
        taffy.add_child(root.taffy_node, child.taffy_node).unwrap();
        // Add the child `LayoutNode` to the root `LayoutNode`'s children vector.
        root.children.push(child);

        // Construct the `LayoutTree` with the Taffy instance and the root `LayoutNode`.
        // Mark it as dirty to ensure layout computation on the first frame.
        let layout_tree = LayoutTree {
            taffy,
            root: Some(root),
            is_dirty: true,
        };

        // Create the `Scene` with the configured `LayoutTree`.
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
    // Initialize the application, performing platform-specific setup (e.g., COM).
    let _app = Application::new()?;
    // Initialize the logger for debugging output.
    env_logger::init();
    log::info!("Hello, Taffy!");

    // Create our application state.
    let app = App::new();

    // Create the root event handler. This handler composes multiple specialized
    // event handlers to process different types of events.
    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    // Add the `DefaultInputHandler` to manage keyboard and mouse input state.
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));
    // Add the `LayoutEventHandler`. This handler is responsible for marking the
    // layout tree as dirty when the window is resized, triggering a re-layout.
    event_handler.add_handler(Box::new(LayoutEventHandler));
    // The `RenderEventHandler` is implicitly added by the framework when `WindowBuilder::build` is called.
    // It handles `Paint` events and draws the scene based on the computed layout.

    // Configure and build the window.
    let window = WindowBuilder::new()
        .with_title("Hello, Taffy!")
        .with_width(900)
        .with_height(600)
        .build(event_handler, app)?;

    // Run the application's main event loop. This will process events,
    // compute layouts, and render the scene until the window is closed.
    window.run()
}
