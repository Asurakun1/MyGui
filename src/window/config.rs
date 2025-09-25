
pub struct WindowConfig {
    pub title: String,
    pub class_name: String,
    pub width: i32,
    pub height: i32,
    pub font_size: i32,
    pub font_face_name: String,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Hello, Windows!".to_string(),
            class_name: "window_class".to_string(),
            width: 800,
            height: 600,
            font_size: 18,
            font_face_name: "MS Gothic".to_string(),
        }
    }
}
