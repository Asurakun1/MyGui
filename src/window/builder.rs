use crate::app::app::App;
use crate::event::event_handler::EventHandler;
use super::config::WindowConfig;
use super::window::Window;
use windows::core::Result;

pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.config.title = title.to_string();
        self
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.config.width = width;
        self
    }

    pub fn with_height(mut self, height: i32) -> Self {
        self.config.height = height;
        self
    }

    pub fn with_font_size(mut self, size: i32) -> Self {
        self.config.font_size = size;
        self
    }

    pub fn with_font_face_name(mut self, name: &str) -> Self {
        self.config.font_face_name = name.to_string();
        self
    }

    pub fn build<E: EventHandler + 'static>(&self, event_handler: E, app: App) -> Result<Box<Window<E>>> {
        Window::new(&self.config, event_handler, app)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
