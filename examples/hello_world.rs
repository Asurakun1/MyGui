//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.
use env_logger;
use my_gui::prelude::*;
use my_gui_core::core::event::handlers::render_event_handler::RenderEventHandler; // Add this
use my_gui_widgets::layout::Layout as MyGuiLayout;
use my_gui_widgets::text_widget::TextWidgetBuilder; // Add this

// 1. Define the application state.
pub struct App {
    pub widget_scene: my_gui_widgets::widget_scene::WidgetScene,
    pub display_text: String,
    pub input_context: InputContext,
}

impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

impl HasDrawableCollection for App {
    fn draw_all_drawables(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        self.widget_scene.draw_all(renderer)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        use my_gui_widgets::rectangle_widget::RectangleWidget;
        use my_gui_widgets::widget_scene::WidgetScene;
        use taffy::prelude::*;
        use taffy::style::Position;

        let mut layout = MyGuiLayout::new(Style {
            size: Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            },
            justify_content: Some(JustifyContent::Center), // Center children horizontally
            align_items: Some(AlignItems::Center),          // Center children vertically
            ..Default::default()
        });

        let display_text = "日本語ハローワールドテスト。".to_string();
        let mut widget_scene = WidgetScene::new();

        // Add a text widget
        let mut text_widget = TextWidgetBuilder::new(0, display_text.clone())
            .with_style(Style {
                size: Size {
                    width: Dimension::length(10.0),
                    height: Dimension::length(10.0),
                },
                position: Position::Absolute,
                ..Default::default()
            })
            .with_color(Color::WHITE)
            .with_font_size(20.0) // Set font size
            .build();
        let text_node_id = layout.add_widget(0, text_widget.get_style().clone(), &[]);
        text_widget.set_layout_node(text_node_id);

        // Add a rectangle widget
        let mut rect_widget = RectangleWidget::new(
            1,
            Style {
                size: Size {
                    width: Dimension::length(200.0),
                    height: Dimension::length(100.0),
                },
                position: Position::Relative,
                ..Default::default()
            },
            Color::WHITE,
            Some(Color::BLUE),
            Some(2.0),
        );
        let rect_node_id = layout.add_widget(1, rect_widget.get_style().clone(), &[]); // Changed widget_id to 1
        rect_widget.set_layout_node(rect_node_id);
        widget_scene.add_widget(Box::new(rect_widget));
        widget_scene.add_widget(Box::new(text_widget));

        // Commenting out Ellipse, Line, and Canvas for now as they are not yet widgets
        // scene.add_object(Ellipse::new(300.0, 100.0, 50.0, 50.0, Color::WHITE));
        // scene.add_object(Ellipse::new(500.0, 100.0, 100.0, 30.0, Color::GREEN));
        // scene.add_object(Line::new(10.0, 170.0, 600.0, 200.0, 2.0, Color::GREEN));
        // let mut canvas = Canvas::new(250.0, 250.0, 300.0, 200.0);
        // canvas.add_object(Rectangle::new(10.0, 10.0, 100.0, 50.0, Color::RED, None, None));
        // canvas.add_object(Ellipse::new(150.0, 100.0, 50.0, 50.0, Color::BLUE));
        // widget_scene.add_widget(Box::new(canvas)); // Canvas is not a widget yet

        layout.compute_layout(Size {
            width: AvailableSpace::Definite(900.0),
            height: AvailableSpace::Definite(600.0),
        });

        widget_scene.update_widget_layouts(&layout);

        Self {
            widget_scene,
            display_text,
            input_context: InputContext::default(),
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
    event_handler.add_handler(Box::new(RenderEventHandler::new())); // Add RenderEventHandler
    event_handler.add_handler(Box::new(CustomEventHandler)); // Add the custom handler

    let window = WindowBuilder::new()
        .with_title("Hello, World! (Not Resizable)")
        .with_width(900)
        .with_height(600)
        .with_resizable(false)
        .with_maximizable(false)
        .build(event_handler, app)?;

    window.run()
}
