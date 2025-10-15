//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use env_logger;
use my_gui::prelude::*;
use taffy::prelude::{AlignItems, Dimension, FlexDirection, JustifyContent, Size, Style, TaffyTree};
// 1. Define the application state.
pub struct App {
    pub input_context: InputContext,
    pub layout_tree: LayoutTree,
}

impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

impl HasLayoutTree for App {
    fn layout_tree(&self) -> &LayoutTree {
        &self.layout_tree
    }

    fn layout_tree_mut(&mut self) -> &mut LayoutTree {
        &mut self.layout_tree
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
            size: Size {
                width: Dimension::length(300.0),
                height: Dimension::length(30.0),
            },
            ..Default::default()
        };
        let text_node = taffy.new_leaf(text_style).unwrap();
        let text = LayoutNode {
            taffy_node: text_node,
            drawable: Some(Box::new(TextObject::new(
                "日本語ハローワールドテスト。".to_string(),
                0.0,
                0.0,
                300.0,
                30.0,
                Color::GREEN,
            ))),
            children: vec![],
        };
        root.children.push(text);
        taffy.add_child(root.taffy_node, text_node).unwrap();

        let rect_style = Style {
            size: Size {
                width: Dimension::length(200.0),
                height: Dimension::length(100.0),
            },
            ..Default::default()
        };
        let rect_node = taffy.new_leaf(rect_style).unwrap();
        let rect = LayoutNode {
            taffy_node: rect_node,
            drawable: Some(Box::new(Rectangle::new(
                0.0,
                0.0,
                200.0,
                100.0,
                Color::WHITE,
            ))),
            children: vec![],
        };
        root.children.push(rect);
        taffy.add_child(root.taffy_node, rect_node).unwrap();

        let ellipse_style = Style {
            size: Size {
                width: Dimension::length(50.0),
                height: Dimension::length(50.0),
            },
            ..Default::default()
        };
        let ellipse_node = taffy.new_leaf(ellipse_style).unwrap();
        let ellipse = LayoutNode {
            taffy_node: ellipse_node,
            drawable: Some(Box::new(Ellipse::new(0.0, 0.0, 25.0, 25.0, Color::WHITE))),
            children: vec![],
        };
        root.children.push(ellipse);
        taffy.add_child(root.taffy_node, ellipse_node).unwrap();

        let stretched_ellipse_style = Style {
            size: Size {
                width: Dimension::length(100.0),
                height: Dimension::length(30.0),
            },
            ..Default::default()
        };
        let stretched_ellipse_node = taffy.new_leaf(stretched_ellipse_style).unwrap();
        let stretched_ellipse = LayoutNode {
            taffy_node: stretched_ellipse_node,
            drawable: Some(Box::new(Ellipse::new(0.0, 0.0, 50.0, 15.0, Color::GREEN))),
            children: vec![],
        };
        root.children.push(stretched_ellipse);
        taffy
            .add_child(root.taffy_node, stretched_ellipse_node)
            .unwrap();

        let line_style = Style {
            size: Size {
                width: Dimension::length(590.0),
                height: Dimension::length(30.0),
            },
            ..Default::default()
        };
        let line_node = taffy.new_leaf(line_style).unwrap();
        let line = LayoutNode {
            taffy_node: line_node,
            drawable: Some(Box::new(my_gui::core::render::objects::primitives::Line::new(
                0.0,
                0.0,
                590.0,
                30.0,
                2.0,
                Color::GREEN,
            ))),
            children: vec![],
        };
        root.children.push(line);
        taffy.add_child(root.taffy_node, line_node).unwrap();

        let canvas_style = Style {
            size: Size {
                width: Dimension::length(300.0),
                height: Dimension::length(200.0),
            },
            ..Default::default()
        };
        let canvas_node = taffy.new_leaf(canvas_style).unwrap();
        let mut canvas = Canvas::new(0.0, 0.0, 300.0, 200.0);
        canvas.add_object(Rectangle::new(10.0, 10.0, 100.0, 50.0, Color::RED));
        canvas.add_object(Ellipse::new(150.0, 100.0, 50.0, 50.0, Color::BLUE));
        let canvas_layout_node = LayoutNode {
            taffy_node: canvas_node,
            drawable: Some(Box::new(canvas)),
            children: vec![],
        };
        root.children.push(canvas_layout_node);
        taffy.add_child(root.taffy_node, canvas_node).unwrap();

        let layout_tree = LayoutTree {
            taffy,
            root: Some(root),
            is_dirty: true,
        };

        Self {
            input_context: InputContext::default(),
            layout_tree,
        }
    }
}

// 2. Define a custom event handler to print key events.
struct CustomEventHandler;

impl EventHandler<App> for CustomEventHandler {
    fn on_event(
        &mut self,
        app: &mut App,
        event: &Event,
        _renderer: &mut dyn Renderer,
    ) -> EventResult {
        let mut result = EventResult::NotConsumed;
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                log::info!(
                    "KeyDown: {:?}, Modifiers: {:?}",
                    key,
                    app.input_context().keyboard
                );
                result = EventResult::Consumed;
            }
            Event::KeyUp(KeyboardEvent { key }) => {
                log::info!(
                    "KeyUp: {:?}, Modifiers: {:?}",
                    key,
                    app.input_context().keyboard
                );
                result = EventResult::Consumed;
            }
            // Event::MouseMove(MouseEvent { x, y, .. }) => {
            //     log::info!("MouseMove: x: {}, y: {}", x, y);
            // }
            Event::MouseDown(MouseEvent { button, .. }) => {
                log::info!("MouseDown: {:?}", button);
                result = EventResult::Consumed;
            }
            Event::MouseUp(MouseEvent { button, .. }) => {
                log::info!("MouseUp: {:?}", button);
                result = EventResult::Consumed;
            }
            Event::MouseWheel(delta) => {
                log::info!("MouseWheel: {:?}", delta);
                result = EventResult::Consumed;
            }
            Event::Character(character) => {
                log::info!("Character: {}", character);
                result = EventResult::Consumed
            }
            Event::WindowClose => {
                log::info!("WindowClose");
                println!("Bye Bye!");
                result = EventResult::Consumed;
            }
            _ => {}
        }

        result
    }
}

// ... (rest of the file is the same until main)

fn main() -> Result<()> {
    let _app = Application::new()?;
    env_logger::init();
    log::info!("Hello, World!");
    // Now returns anyhow::Result
    let app = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));
    event_handler.add_handler(Box::new(CustomEventHandler)); // Add the custom handler
    event_handler.add_handler(Box::new(LayoutEventHandler));

    let window = WindowBuilder::new()
        .with_title("Hello, World! (Not Resizable)")
        .with_width(900)
        .with_height(600)
        .with_resizable(false)
        .with_maximizable(false)
        .build(event_handler, app)?;

    window.run()
}
