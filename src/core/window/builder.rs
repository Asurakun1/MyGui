use crate::core::event::input_state::HasInputState;
use anyhow::Result;
use crate::core::event::event_handler::EventHandler;
use crate::core::window::config::WindowConfig;
use crate::core::platform::window_backend::WindowBackend;

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

    /// Creates a new `WindowBuilder` with the given configuration.
    pub fn with_config(config: WindowConfig) -> Self {
        Self { config }
    }

    /// Creates a new `WindowBuilder` from the given configuration.
    pub fn from_config(config: WindowConfig) -> Self {
        Self::with_config(config)
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
    pub fn build<T: 'static + HasInputState, E: EventHandler<T> + 'static>(
        &self,
        event_handler: E,
        app: T,
    ) -> Result<Box<dyn WindowBackend<T, E>>> {
        #[cfg(target_os = "windows")]
        {
            use crate::core::platform::win32_window::Win32Window;
            let backend = Win32Window::new(&self.config, event_handler, app)?;
            Ok(backend)
        }

        #[cfg(not(target_os = "windows"))]
        {
            panic!("Unsupported operating system");
        }
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
