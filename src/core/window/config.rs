use crate::core::backend::renderer::RendererConfig; // Import RendererConfig

/// Configuration for a window.
///
/// This struct holds all the settings for a window, such as its title, size,
/// and font.
pub struct WindowConfig {
    /// The title of the window.
    pub title: String,
    /// The name of the window class.
    pub class_name: String,
    /// The width of the window.
    pub width: i32,
    /// The height of the window.
    pub height: i32,
    /// The font size for the window.
    pub font_size: i32,
    /// The font face name for the window.
    pub font_face_name: String,
    /// The renderer configuration for the window.
    pub renderer_config: RendererConfig, // New field
}

impl Default for WindowConfig {
    /// Creates a new `WindowConfig` with default settings.
    fn default() -> Self {
        Self {
            title: "Hello, Windows!".to_string(),
            class_name: "window_class".to_string(),
            width: 800,
            height: 600,
            font_size: 18,
            font_face_name: "MS Gothic".to_string(),
            renderer_config: RendererConfig::Direct2D, // Default to Direct2D
        }
    }
}
