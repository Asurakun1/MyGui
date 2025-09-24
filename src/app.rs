use crate::render::{objects::text_object::TextObject, scene::Scene};

pub struct App {
    pub scene: Scene,
    pub display_text: String,
}

impl App {
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
