use crate::render::{objects::text_object::TextObject, scene::Scene};

/// Represents the central state of the application.
///
/// This struct holds all the data that defines the application's current state,
/// such as the objects to be rendered and other application-specific data. It is
/// owned by the `Window` and passed mutably to event handlers, allowing them to
/// inspect and modify the state in response to events.
/// Represents the central state of the application.
///
/// This struct holds all the data that defines the application's current state,
/// such as the objects to be rendered and other application-specific data. It is
/// owned by the `Window` and passed mutably to event handlers, allowing them to
/// inspect and modify the state in response to events.
pub struct App {
    /// The scene containing all drawable objects.
    pub scene: Scene,
    /// The text string to be displayed in the window.
    pub display_text: String,
}

impl App {
    /// Creates a new `App` instance with default state.
    ///
    /// This initializes the application with a sample scene containing a text object.
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
