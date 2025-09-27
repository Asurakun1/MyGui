use crate::app::App;
use crate::event::event_handler::EventHandler;
use super::config::WindowConfig;
use super::Window;
use windows::core::Result;

/// A builder for creating and configuring a `Window`.
///
/// This struct provides a fluent interface for setting window properties.
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    /// Creates a new `WindowBuilder` with default settings.
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    /// Sets the title of the window.
    pub fn with_title(mut self, title: &str) -> Self {
        self.config.title = title.to_string();
        self
    }

    /// Sets the width of the window.
    pub fn with_width(mut self, width: i32) -> Self {
        self.config.width = width;
        self
    }

    /// Sets the height of the window.
    pub fn with_height(mut self, height: i32) -> Self {
        self.config.height = height;
        self
    }

    /// Sets the font size for the window.
    pub fn with_font_size(mut self, size: i32) -> Self {
        self.config.font_size = size;
        self
    }

    /// Sets the font face name for the window.
    pub fn with_font_face_name(mut self, name: &str) -> Self {
        self.config.font_face_name = name.to_string();
        self
    }

    /// Builds the window.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to create the window.
    pub fn build<E: EventHandler + 'static>(&self, event_handler: E, app: App) -> Result<Box<Window<E>>> {
        Window::new(&self.config, event_handler, app)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}