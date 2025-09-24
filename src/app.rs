use crate::render::{objects::text_object::TextObject, scene::Scene};

/// Represents the application state.
///
/// This struct holds the data that the application needs to maintain its state,
/// such as the scene to be rendered.
pub struct App {
    /// The scene containing the objects to be rendered.
    pub scene: Scene,
    /// The text to be displayed.
    pub display_text: String,
}

impl App {
    /// Creates a new `App` with a default scene.
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
